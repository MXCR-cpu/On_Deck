use ecies::encrypt;
use interact::site::SITE_LINK;
use mechanics::board::PositionVectors;
use mechanics::position::FirePosition;
use mechanics::position::FiredState;
use mechanics::ship::Ship;
use utils_files::event_source_state::EventSourceState;
use utils_files::request::fire_on_position;
use utils_files::request::get_request;
use utils_files::web_error::ClientError;
use wasm_bindgen::JsValue;
use yew::classes;
use yew::html;
use yew::Context;
use yew::Html;
use yew::{Component, Properties};

pub struct Board {
    board: Option<PositionVectors>,
    ships: Option<Vec<Ship>>,
    player_titles: Option<Vec<String>>,
    player_index: Option<usize>,
    challenge: Option<String>,
    round: u32,
    event_source: EventSourceState,
}

pub enum BoardMsg {
    AwaitUpdate,
    Update((String, String, String, String)),
    EndUpdate,
    Fire(usize, usize, usize),
    Response(ClientError),
}

#[derive(Properties, PartialEq)]
pub struct BoardProp {
    pub access_key: String,
    pub player_id_key: String,
    pub player_id_tag: String,
    pub game_number: u32,
}

impl Component for Board {
    type Message = BoardMsg;
    type Properties = BoardProp;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let callback_update = ctx.link().callback(move |_: ()| Self::Message::AwaitUpdate);
        let callback_end = ctx.link().callback(move |_: ()| Self::Message::EndUpdate);
        let event_source: EventSourceState = EventSourceState::new(
            &format!("{}/game/{}/game_stream", SITE_LINK, ctx.props().game_number),
            None,
            move |_| callback_update.emit(()),
            move |_| callback_end.emit(()),
        );
        Self {
            board: None,
            ships: None,
            player_titles: None,
            player_index: None,
            challenge: None,
            round: 0,
            event_source,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::AwaitUpdate => {
                self.send_update_request(ctx);
            }
            Self::Message::Update((challenge, player_titles, ship_positions, game_state)) => {
                self.challenge = if challenge.is_empty() {
                    None
                } else {
                    if !self
                        .challenge
                        .clone()
                        .unwrap_or("".to_string())
                        .eq(&challenge)
                    {
                        self.round += 1;
                    }
                    Some(challenge.to_string())
                };
                self.player_titles = if player_titles.is_empty() {
                    self.player_index = None;
                    None
                } else {
                    let player_titles_vec: Vec<String> =
                        serde_json::from_str(&player_titles).unwrap();
                    self.player_index = Some(
                        player_titles_vec
                            .iter()
                            .position(|x| ctx.props().player_id_tag.eq(x))
                            .unwrap_or(0),
                    );
                    Some(player_titles_vec)
                };
                self.ships = if ship_positions.is_empty() {
                    None
                } else {
                    Some(serde_json::from_str::<Vec<Ship>>(&ship_positions).unwrap())
                };
                self.board = if game_state.is_empty() {
                    None
                } else {
                    Some(serde_json::from_str::<PositionVectors>(&game_state).unwrap())
                };
            }
            Self::Message::EndUpdate => {
                self.event_source.close_connection();
            }
            Self::Message::Fire(x_pos, y_pos, to) => {
                let message_bytes: Vec<u8> = encrypt(
                    &serde_json::from_str::<Vec<u8>>(&ctx.props().player_id_key.clone()).unwrap(),
                    &self.challenge.clone().unwrap().as_bytes(),
                )
                .unwrap();
                let player_index: usize = self.player_index.clone().unwrap();
                let game_number: u32 = ctx.props().game_number;
                ctx.link().send_future(async move {
                    match fire_on_position::<FirePosition>(
                        FirePosition::new(message_bytes, player_index, to, x_pos, y_pos),
                        game_number,
                    )
                    .await
                    {
                        Ok(_) => Self::Message::Response(ClientError::from(
                            file!(),
                            "update(): fire request sent",
                        )),
                        Err(error) => Self::Message::Response(ClientError::from(
                            file!(),
                            &format!("update(): could not send fire post request: {}", error),
                        )),
                    }
                });
            }
            Self::Message::Response(error) => {
                web_sys::console::log_1(&JsValue::from(format!("{}", error)));
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let indecies: Vec<(usize, usize)> = (0..10)
            .map(|index| {
                (0..10)
                    .map(|jndex| (index, jndex))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<Vec<(usize, usize)>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<(usize, usize)>>();
        // This is not memory performant
        let onclick = |index: usize, jndex: usize, to: usize| {
            _ctx.link()
                .callback(move |_| Self::Message::Fire(index, jndex, to))
        };
        let map_button_class = |result: &str, index: usize, jndex: usize| {
            classes!(
                "main_button",
                format!("main_button_{}", result),
                format!("button_row_{}", 9 - jndex),
                format!("button_col_{}", index)
            )
        };
        let number_of_players: usize = match &self.board.clone() {
            Some(board) => board[0][0].fired_state.len(),
            None => 0,
        };
        let player_titles_unwrapped: Vec<String> = self.player_titles.clone().unwrap_or(Vec::new());
        let player_index_unwrapped: usize = self.player_index.clone().unwrap_or(0);
        html! {
            <div id={"Board_Component"}>
                <div id={"Round_Heading"}>
                    if self.round != 0 {
                        <h2 class={classes!("round_title", "font")}>{ format!("Round {}", self.round) }</h2>
                    } else {
                        <h2 class={classes!("round_title", "font")}>{ "Game Not Yet Started" }</h2>
                    }
                </div>
                <div class={"battlefield"}>{
                    (0..number_of_players)
                        .into_iter()
                        .map(|index: usize| html! {
                            <div id={format!("{}", player_titles_unwrapped[index])}>
                                <div class={"board"}>{
                                    if let Some(board) = self.board.clone() {
                                        indecies.clone()
                                            .into_iter()
                                            .map(|(x_pos, y_pos): (usize, usize)| {
                                                match &board[x_pos][y_pos].fired_state[index] {
                                                    FiredState::Untouched => {
                                                        if index == player_index_unwrapped {
                                                            return html! {
                                                                <button class={map_button_class("empty", x_pos, y_pos)} />
                                                            };
                                                        }
                                                        html! {
                                                            <button class={map_button_class("untouched", x_pos, y_pos)} onclick={onclick(x_pos, y_pos, index)} />
                                                        }
                                                    }
                                                    FiredState::Miss => {
                                                        html! {
                                                            <button class={map_button_class("miss", x_pos, y_pos)} />
                                                        }
                                                    }
                                                    FiredState::Hit => {
                                                        html! {
                                                            <button class={map_button_class("hit", x_pos, y_pos)} />
                                                        }
                                                    }
                                                    FiredState::Empty => {
                                                        html! {
                                                            <button class={map_button_class("empty", x_pos, y_pos)} />
                                                        }
                                                    }
                                                    FiredState::Ship(ship_type) => {
                                                        html! {
                                                            <button class={classes!(map_button_class("ship", x_pos, y_pos), ship_type)}>
                                                                <img src={format!("http://127.0.0.1:8000/extra_files/ships_{ship_type}_day.svg")} />
                                                            </button>
                                                        }
                                                    }
                                                }
                                            })
                                            .collect::<Html>()
                                    } else {
                                        html! {
                                            <p>{ "boards are loading ..." }</p>
                                        }
                                    }
                                }</div>
                                if index < player_titles_unwrapped.len() {
                                    <h3 class={classes!("font", "player_title")}>{ format!("{}", player_titles_unwrapped[index]) }</h3>
                                } else {
                                    <h3 class={classes!("font", "player_title")}>{ "Empty" }</h3>
                                }
                            </div>
                        })
                        .collect::<Html>()
                }
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            _ctx.link()
                .send_message(Self::Message::Response(ClientError::from(
                    file!(),
                    "rendered(): rendered board component",
                )));
        }
    }

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &yew::Context<Self>) {
        self.event_source.close_connection();
    }
}

impl Board {
    fn send_update_request(&self, _ctx: &Context<Self>) {
        let game_number: u32 = _ctx.props().game_number;
        let player_id: String = _ctx.props().player_id_tag.clone();
        let access_message: String = _ctx.props().access_key.clone();
        _ctx.link().send_future(async move {
            match get_request::<(String, String, String, String)>(
                format!(
                    "{SITE_LINK}/game/{}/{}/{}",
                    game_number, player_id, access_message
                )
                .as_str(),
            )
            .await
            {
                Ok(result) => BoardMsg::Update(result),
                Err(error) => BoardMsg::Response(error.push(
                    file!(),
                    "send_update_request(): Could not receive active game link update",
                )),
            }
        });
    }
}

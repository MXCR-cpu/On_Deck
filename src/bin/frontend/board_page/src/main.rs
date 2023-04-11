// use wasm_bindgen::prelude::*;
use ecies::encrypt;
use interact::site::SITE_LINK;
use mechanics::board::PositionVectors;
use mechanics::position::FirePosition;
use mechanics::position::FiredState;
use mechanics::ship::Ship;
use regex::Regex;
use std::time::Duration;
use utils_files::request::fire_on_position;
use utils_files::request::get_request;
use utils_files::sky::Sky;
use utils_files::window_state::ClientWindow;
use wasm_bindgen::JsValue;
use yew::classes;
use yew::platform::time::sleep;
use yew::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     pub fn btoa(data_string: &str) -> String;
//     pub fn atob(data_vector: &str) -> String;
// }

struct ClientGame {
    client_window: ClientWindow,
    boards: Option<PositionVectors>,
    ships: Option<Vec<Ship>>,
    access_message: String,
    challenge: Option<String>,
    game_number: u32,
}

enum ClientGameMsg {
    Fire(usize, usize, usize),
    Fired,
    NotFired,
    NotReceived,
    UpdateBoardGame((String, String, String)),
}

impl Component for ClientGame {
    type Message = ClientGameMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let client_window: ClientWindow = match ClientWindow::new() {
            Ok(window_state) => window_state,
            Err(error) => {
                web_sys::console::log_1(&JsValue::from(format!(
                    "board_page/src/main.rs: create(): {error};"
                )));
                panic!()
            }
        };
        let game_number: u32 = Self::retreive_game_number(&client_window);
        let encryption_key: Vec<u8> =
            serde_json::from_str::<Vec<u8>>(&client_window.player_id_key.clone().unwrap()).unwrap();
        // web_sys::console::log_1(&JsValue::from(format!(
        //     "Key Vector (length {}): {:?}",
        //     encryption_key.len(),
        //     encryption_key
        // )));
        let access_message: String = encrypt(
                    &encryption_key,
                    String::from("Request").as_bytes(),
                    )
                .unwrap_or_else(|error| {
                    web_sys::console::log_1(&JsValue::from(format!(
                                "board_page/src/main.rs: create(): Failed to create access_message \n\t{}: Uncoded Key (Length {}):\n{:?}",
                                error.to_string(),
                                encryption_key.len(),
                                encryption_key,
                                )));
                    panic!()
                })
                .into_iter()
                .map(|element: u8| format!("{:x}", element))
                .collect::<Vec<String>>()
                .join("");
        let client_game: Self = Self {
            client_window,
            boards: None,
            ships: None,
            access_message,
            challenge: None,
            game_number,
        };
        client_game.send_update_request(&_ctx, 0);
        client_game
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let game_number: u32 = self.game_number;
        match msg {
            Self::Message::Fire(x_pos, y_pos, to) => {
                let message_bytes: Vec<u8> = encrypt(
                    self.client_window.player_id_key.clone().unwrap().as_bytes(),
                    &self.challenge.clone().unwrap().as_bytes().to_vec(),
                )
                .unwrap();
                let sent_player_tag: String = self.client_window.player_id_tag.clone().unwrap();
                _ctx.link().send_future(async move {
                    match fire_on_position::<FirePosition>(
                        FirePosition::new(message_bytes, sent_player_tag, x_pos, y_pos, to),
                        game_number,
                        )
                        .await
                        {
                            Ok(_) => Self::Message::Fired,
                            Err(error) => {
                                web_sys::console::log_1(
                                    &format!(
                                        "board_page/src/main.rs: update(): Could send fire post request; \n\t{}",
                                        error
                                        ).into()
                                    );
                                Self::Message::NotFired
                            }
                        }
                });
            }
            Self::Message::UpdateBoardGame((challenge, ship_positions, game_state)) => {
                self.challenge = if challenge.is_empty() {
                    None
                } else {
                    Some(challenge.to_string())
                };
                self.ships = if ship_positions.is_empty() {
                    None
                } else {
                    Some(serde_json::from_str::<Vec<Ship>>(&ship_positions).unwrap())
                };
                self.boards = if game_state.is_empty() {
                    None
                } else {
                    Some(serde_json::from_str::<PositionVectors>(&game_state).unwrap())
                };
                self.send_update_request(_ctx, 5);
            }
            _ => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
        let number_of_players: usize = match &self.boards.clone() {
            Some(board) => board[0][0].fired_state.len(),
            None => 0,
        };
        let onclick = |index: usize, jndex: usize, to: usize| {
            ctx.link()
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
        html! {
            <div class={classes!("sky_whole", if self.client_window.day { "sky_day" } else { "sky_night" })}>
                if !self.client_window.day {
                    <Sky max_stars={20} star_size={2} log={false} />
                }
                <div class={classes!("ocean_setting", if self.client_window.day { "ocean_day" } else { "ocean_night" })}>
                    <div class={"battlefield"}>{
                        (0..number_of_players)
                            .into_iter()
                            .map(|index: usize| html! {
                                <div class={"board"}>{
                                    if let Some(board) = self.boards.clone() {
                                        indecies.clone()
                                            .into_iter()
                                            .map(|(x_pos, y_pos): (usize, usize)| {
                                                match board[x_pos][y_pos].fired_state[index] {
                                                    FiredState::Untouched => {
                                                        html! {
                                                            <button class={map_button_class("untouched", x_pos, y_pos)} onclick={onclick(x_pos, y_pos, index)}></button>
                                                        }
                                                    }
                                                    FiredState::Miss => {
                                                        html! {
                                                            <button class={map_button_class("miss", x_pos, y_pos)}></button>
                                                        }
                                                    }
                                                    FiredState::Hit => {
                                                        html! {
                                                            <button class={map_button_class("hit", x_pos, y_pos)}></button>
                                                        }
                                                    }
                                                    FiredState::Empty => {
                                                        html! {
                                                            <button class={map_button_class("empty", x_pos, y_pos)}></button>
                                                        }
                                                    }
                                                    FiredState::Ship => {
                                                        html! {
                                                            <button class={map_button_class("ship", x_pos, y_pos)}></button>
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
                            })
                            .collect::<Html>()
                    }
                    </div>
                </div>
            </div>
        }
    }
}

impl ClientGame {
    fn retreive_game_number(client_window: &ClientWindow) -> u32 {
        match Regex::new(r"\d+").unwrap().find(
            Regex::new(r"game/\d+")
                .unwrap()
                .find(&client_window.window.location().href().unwrap())
                .unwrap()
                .as_str(),
        ) {
            Some(result) => match result.as_str().parse::<u32>() {
                Ok(result) => result,
                Err(error) => {
                    web_sys::console::log_1(
                        &format!("board_page/src/main.rs: create(): Could not parse &str into u32; \n\t{}", error).into()
                    );
                    panic!()
                }
            },
            None => {
                web_sys::console::log_1(
                        &format!("board_page/src/main.rs: create(): Regex did not find any matching patterns for game_id within the url").into());
                panic!()
            }
        }
    }

    fn send_update_request(&self, _ctx: &Context<Self>, waiting_time: u64) {
        let game_number: u32 = self.game_number;
        let player_id: String = self.client_window.player_id_tag.clone().unwrap();
        let access_message: String = self.access_message.clone();
        _ctx.link().send_future(async move {
            sleep(Duration::from_secs(waiting_time)).await;
            match get_request::<(String, String, String)>(format!("{SITE_LINK}/game/{}/{}/{}", game_number, player_id, access_message).as_str()).await
            {
                Ok(result) => ClientGameMsg::UpdateBoardGame(result),
                Err(error) => {
                    web_sys::console::log_1(
                        &JsValue::from(
                            format!("board_page/src/main.rs: send_update_request(): Could not receive active game link update; \n\t{}", error))
                    );
                    ClientGameMsg::NotReceived
                }
            }
        });
    }
}

fn main() {
    yew::Renderer::<ClientGame>::new().render();
}

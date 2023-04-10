use ecies::encrypt;
use interact::site::SITE_LINK;
use mechanics::board::PositionVectors;
use mechanics::position::FirePosition;
use mechanics::position::FiredState;
use regex::Regex;
use std::time::Duration;
use utils_files::request::fire_on_position;
use utils_files::request::get_request;
#[allow(unused_imports)]
use utils_files::sky::Sky;
use utils_files::window_state::ClientWindow;
use wasm_bindgen::JsValue;
use yew::classes;
use yew::platform::time::sleep;
use yew::prelude::*;

struct ClientGame {
    client_window: ClientWindow,
    boards: PositionVectors,
    challenge: String,
    game_number: u32,
}

enum ClientGameMsg {
    Fire(usize, usize, usize),
    Fired,
    NotFired,
    NotReceived,
    UpdateBoardGame((String, String)),
}

impl Component for ClientGame {
    type Message = ClientGameMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let client_window: ClientWindow = match ClientWindow::new() {
            Ok(window_state) => window_state,
            Err(error) => {
                web_sys::console::log_1(&JsValue::from(error));
                panic!()
            }
        };
        let game_number: u32 = Self::retreive_game_number(&client_window);
        Self::send_update_request(_ctx, game_number, 0);
        Self {
            client_window,
            boards: Vec::new(),
            challenge: "".to_string(),
            game_number,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let game_number: u32 = self.game_number;
        match msg {
            Self::Message::Fire(x_pos, y_pos, to) => {
                let message_bytes: Vec<u8> = encrypt(
                    self.client_window.player_id_key.clone().unwrap().as_bytes(),
                    &self.challenge.as_bytes().to_vec(),
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
            Self::Message::UpdateBoardGame((challenge, game_state)) => {
                self.boards = serde_json::from_str(&game_state).unwrap();
                self.challenge = challenge;
                Self::send_update_request(_ctx, game_number, 5);
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
        let number_of_players: &usize = &self.boards[0][0].fired_state.len();
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
                <div class={classes!("ocean_setting", if self.client_window.day { "ocean_day" } else { "ocean_night" })}>
                    <div class={"battlefield"}>{
                        (0..*number_of_players)
                            .into_iter()
                            .map(|index: usize| html! {
                                <div class={"board"}>{
                                    indecies.clone()
                                        .into_iter()
                                        .map(|(x_pos, y_pos): (usize, usize)| {
                                            match self.boards[x_pos][y_pos].fired_state[index] {
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

    fn send_update_request(_ctx: &Context<Self>, game_number: u32, waiting_time: u64) {
        _ctx.link().send_future(async move {
            sleep(Duration::from_secs(waiting_time)).await;
            match get_request::<(String, String)>(format!("{}/game/{}", SITE_LINK, game_number).as_str()).await
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

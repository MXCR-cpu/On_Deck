use frontend::board::Board;
use frontend::position::FirePosition;
use frontend::position::FiredState;
use frontend::site::SITE_LINK;
use regex::Regex;
use std::time::Duration;
use utils_files::request::fire_on_position;
use utils_files::request::get_request;
#[allow(unused_imports)]
use utils_files::sky::Sky;
use wasm_bindgen::JsValue;
use web_sys::{Storage, Window};
use yew::classes;
use yew::platform::time::sleep;
use yew::prelude::*;

struct ClientGame {
    day: bool,
    boards: Board,
    player_id: String,
    game_number: u32,
}

enum ClientGameMsg {
    Fire(usize, usize, usize),
    Fired,
    NotFired,
    ReceivedId(String),
    NotReceived,
    UpdateBoardGame(Board),
    Sent,
}

impl Component for ClientGame {
    type Message = ClientGameMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let window: Window = match web_sys::window() {
            Some(window) => window,
            None => {
                web_sys::console::log_1(&JsValue::from(
                    "board_page/src/main.rs: create(): Window object not found; ",
                ));
                panic!()
            }
        };
        let storage: Storage = match window.local_storage() {
            Ok(option) => match option {
                Some(storage) => storage,
                None => {
                    web_sys::console::log_1(&JsValue::from(
                        "board_page/src/main.rs: create(): Storage object not found; ",
                    ));
                    panic!()
                }
            },
            Err(error) => {
                web_sys::console::log_2(
                    &JsValue::from("board_page/src/main.rs: create(): Storage object not found; \n\t"),
                    &error,
                );
                panic!()
            }
        };
        let player_id: String = match storage.get_item("player_id") {
            Ok(value) => match value {
                Some(inner_player_id) => inner_player_id.to_string(),
                None => {
                    web_sys::console::log_1(&JsValue::from(
                        "board_page/src/main.rs: create(), 68: player_id item not found, retrieving new player_id...",
                    ));
                    _ctx.link().send_future(async move {
                        match get_request::<u32>(format!("{}/get_player_id", SITE_LINK).as_str())
                            .await
                        {
                            Ok(player_id) => Self::Message::ReceivedId(player_id.to_string()),
                            Err(_) => Self::Message::NotReceived,
                        }
                    });
                    String::new()
                }
            },
            Err(error) => {
                web_sys::console::log_2(
                    &JsValue::from("board_page/src/main.rs: create(): Storage object not found; \n\t"),
                    &error,
                );
                panic!()
            }
        };
        let day: bool = match storage.get_item("day_setting") {
            Ok(value) => match value
                .unwrap_or_else(|| {
                    storage.set_item("day_setting", "day").unwrap();
                    "day".to_string()
                })
                .as_str()
            {
                "day" => true,
                "night" => false,
                _ => false,
            },
            Err(error) => {
                web_sys::console::log_2(
                    &JsValue::from("board_page/src/main.rs: create(): Could not access storage; \n\t"),
                    &error,
                );
                panic!()
            }
        };
        let game_number: u32 = match Regex::new(r"\d+").unwrap().find(
            Regex::new(r"game/\d+")
                .unwrap()
                .find(&window.location().href().unwrap())
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
        };
        _ctx.link().send_future(async move {
            match get_request::<Board>(format!("{}/game/{}", SITE_LINK, game_number).as_str()).await
            {
                Ok(result) => Self::Message::UpdateBoardGame(result),
                Err(error) => {
                    web_sys::console::log_1(
                        &JsValue::from(
                            format!("board_page/src/main.rs: create(): Could not receive active game link update; \n\t{}", error))
                    );
                    Self::Message::NotReceived
                }
            }
        });
        Self {
            boards: Board::empty(),
            day,
            player_id,
            game_number,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let game_number: u32 = self.game_number;
        match msg {
            Self::Message::Fire(x_pos, y_pos, to) => _ctx.link().send_future(async move {
                match fire_on_position::<FirePosition>(
                    FirePosition::new(x_pos, y_pos, to),
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
            }),
            Self::Message::ReceivedId(player_id) => {
                self.player_id = player_id;
            }
            Self::Message::UpdateBoardGame(game_state) => {
                self.boards = game_state;
                _ctx.link().send_message(Self::Message::Sent);
            }
            Self::Message::Sent => {
                _ctx.link().send_future(async move {
                    sleep(Duration::from_secs(5)).await;
                        match get_request::<Board>(
                            format!("{}/game/{}", SITE_LINK, game_number).as_str(),
                        )
                        .await {
                            Ok(result) => Self::Message::UpdateBoardGame(result),
                            Err(error) => {
                                web_sys::console::log_1(
                                    &format!(
                                        "board_page/src/main.rs: update(): Could not receive active game link update; \n\t{}",
                                        error
                                    ).into()
                                );
                                Self::Message::NotReceived
                            }
                        }
                });
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
        let number_of_players: &usize = &self.boards.board[0][0].fired_state.len();
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
            <div>
                <div class={classes!("ocean_setting", if self.day { "sky_day" } else { "sky_night" })}>
                    <div class={"battlefield"}>{
                        (0..*number_of_players)
                            .into_iter()
                            .map(|index: usize| html! {
                                <div class={"board"}>{
                                    indecies.clone()
                                        .into_iter()
                                        .map(|(x_pos, y_pos): (usize, usize)| {
                                            match self.boards.board[x_pos][y_pos].fired_state[index] {
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

fn main() {
    yew::Renderer::<ClientGame>::new().render();
}

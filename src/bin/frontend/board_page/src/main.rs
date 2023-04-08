use mechanics::board::Board;
use mechanics::position::FirePosition;
use mechanics::position::FiredState;
use interact::site::SITE_LINK;
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
    boards: Board,
    game_number: u32,
}

enum ClientGameMsg {
    Fire(usize, usize, usize),
    Fired,
    NotFired,
    NotReceived,
    UpdateBoardGame(Board),
    Sent,
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
        let game_number: u32 = match Regex::new(r"\d+").unwrap().find(
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
            client_window,
            boards: Board::empty(),
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
            <div class={classes!("sky_whole", if self.client_window.day { "sky_day" } else { "sky_night" })}>
                <div class={"ocean_setting"}>
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

fn main() {
    yew::Renderer::<ClientGame>::new().render();
}

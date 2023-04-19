use board_component::Board;
use ecies::encrypt;
use ecies::SecpError;
use regex::Regex;
use utils_files::sky::Stars;
use utils_files::web_error::ClientError;
use utils_files::window_state::ClientWindow;
use wasm_bindgen::JsValue;
use yew::classes;
use yew::prelude::*;

mod board_component;

struct ClientGame {
    client_window: ClientWindow,
    access_message: String,
    game_number: u32,
}

enum ClientGameMsg {
    Response(ClientError),
}

impl Component for ClientGame {
    type Message = ClientGameMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let client_window: ClientWindow = match ClientWindow::new() {
            Ok(window_state) => window_state,
            Err(error) => {
                _ctx.link().send_message(Self::Message::Response(
                    error.push(file!(), "create(): Failed to create ClientWindow"),
                ));
                panic!();
            }
        };
        let game_number: u32 =
            Self::retreive_game_number(&client_window).unwrap_or_else(|error: ClientError| {
                _ctx.link().send_message(Self::Message::Response(error));
                0
            });
        let encryption_key: Vec<u8> =
            serde_json::from_str::<Vec<u8>>(&client_window.player_id_key.clone().unwrap_or_else(|| {
                _ctx.link().send_message(Self::Message::Response(
                    ClientError::new().push(
                        file!(),
                        &format!("create(): failed to clone and unwrap the player_id_key field of client_window"))));
                panic!()
            })).unwrap_or_else(|_| {
                _ctx.link().send_message(Self::Message::Response(
                    ClientError::new().push(file!(), "create(): failed to parse string with serde_json")));
                panic!()
            });
        let access_message: String = encrypt(&encryption_key, String::from("Request").as_bytes())
            .unwrap_or_else(|error: SecpError| {
                _ctx.link().send_message(Self::Message::Response(ClientError::new().push(
                    file!(),
                    &format!(
                        "create(): Failed to create access_message: {}: uncoded key (Length {}):{:?}",
                        error,
                        encryption_key.len(),
                        encryption_key,
                    ),
                )));
                panic!()
            })
            .into_iter()
            .map(|element: u8| format!("{:02x}", element))
            .collect::<Vec<String>>()
            .join("");
        Self {
            client_window,
            access_message,
            game_number,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("sky_whole", if self.client_window.day { "sky_day" } else { "sky_night" })}>
                if !self.client_window.day {
                    <Stars max_stars={20} star_size={2} log={false} />
                }
                <div class={classes!("ocean_setting", if self.client_window.day { "ocean_day" } else { "ocean_night" })}>
                    <Board
                        access_key={self.access_message.clone()}
                        player_id_key={self.client_window.player_id_key.clone().unwrap()}
                        player_id_tag={self.client_window.player_id_tag.clone().unwrap()}
                        game_number={self.game_number} />
                </div>
            </div>
        }
    }
}

impl ClientGame {
    fn retreive_game_number(client_window: &ClientWindow) -> Result<u32, ClientError> {
        Regex::new(r"\d+").unwrap().find(
            Regex::new(r"game/\d+")
                .unwrap()
                .find(&client_window.window.location().href().map_err(|error: JsValue| {
                    ClientError::from(file!(), &format!("retreive_game_number(): failed to get window location: {:?}", error))
                })?)
                .ok_or(ClientError::from(file!(), &format!("retreive_game_number(): failed to conduct regex operation")))?
                .as_str(),
        ).ok_or(ClientError::from(file!(), &"retreive_game_number(): regex did not find any matching patterns for game_id within the url".to_string()))?
        .as_str()
        .parse::<u32>()
        .map_err(|error: _|{
            ClientError::from(file!(), &format!("retreive_game_number(): Could not parse &str into u32: {}", error))
        })
    }
}

fn main() {
    yew::Renderer::<ClientGame>::new().render();
}

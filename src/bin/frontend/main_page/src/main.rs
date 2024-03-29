use crate::navbar_component::Navbar;
use crate::panel_component::Pages;
use crate::panel_component::Panel;
use interact::site::SITE_LINK;
use utils_files::animation_level::AnimationLevel;
use utils_files::request::get_request;
use utils_files::sky::Clouds;
use utils_files::sky::Stars;
use utils_files::web_error::ClientError;
use utils_files::window_state::ClientWindow;
use wasm_bindgen::JsValue;
use yew::classes;
use yew::html;
use yew::prelude::*;

mod navbar_component;
mod panel_component;

#[allow(dead_code)]
pub struct Menu {
    client_window: ClientWindow,
    page_selection: Pages,
}

#[derive(Clone)]
pub enum MenuMsg {
    ChangeDayState,
    ChangePage,
    ChangePlayerId(String),
    ChangeAnimationLevel(AnimationLevel),
    ReloadPage,
    ReceivedId((String, String)),
    Response(ClientError),
    None,
}

impl Component for Menu {
    type Message = MenuMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let client_window: ClientWindow =
            ClientWindow::new().unwrap_or_else(|error: ClientError| {
                _ctx.link().send_message(Self::Message::Response(
                    error.push(file!(), "create(): failed to create ClientWindow"),
                ));
                panic!();
            });
        client_window.player_id_tag.clone().unwrap_or_else(|| {
            _ctx.link()
                .send_message(Self::Message::Response(ClientError::from(
                    file!(),
                    "board_page: create(): Getting new player_id",
                )));
            _ctx.link().send_future(async move {
                match get_request::<(String, String)>(&format!("{SITE_LINK}/get_player_id")).await {
                    Ok(result) => Self::Message::ReceivedId(result),
                    Err(error) => Self::Message::Response(error.push(
                        file!(),
                        "create(): client_window.player_id_tag failed to unwrap",
                    )),
                }
            });
            String::new()
        });
        Self {
            client_window,
            page_selection: Pages::Main,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Self::Message::ChangeDayState => {
                self.client_window.day = !self.client_window.day;
                self.client_window
                    .local_storage
                    .set_item(
                        "day_setting",
                        if self.client_window.day {
                            "day"
                        } else {
                            "night"
                        },
                    )
                    .unwrap();
                true
            }
            Self::Message::ChangePage => {
                self.page_selection = match self.page_selection {
                    Pages::Main => Pages::Settings,
                    Pages::Settings => Pages::Main,
                };
                true
            }
            Self::Message::ChangePlayerId(new_player_id) => {
                match self.client_window.set_player_id_tag(new_player_id) {
                    Ok(()) => (),
                    Err(error) => _ctx.link().send_message(Self::Message::Response(error)),
                };

                false
            }
            Self::Message::ChangeAnimationLevel(new_level) => {
                match self.client_window.set_animation_level(new_level) {
                    Ok(()) => (),
                    Err(error) => _ctx.link().send_message(Self::Message::Response(error)),
                }
                true
            }
            Self::Message::ReloadPage => {
                match self.client_window.window.location().reload() {
                    Ok(()) => (),
                    Err(error) => {
                        _ctx.link()
                            .send_message(Self::Message::Response(ClientError::from(
                                file!(),
                                &format!("update(): Failed to reload page: {:?}", error),
                            )))
                    }
                };
                false
            }
            Self::Message::ReloadPage => match self.client_window.window.location().reload() {
                Ok(()) => (),
                Err(error) => _ctx
                    .link()
                    .send_message(Self::Message::Response(ClientError::from(
                        file!(),
                        &format!("update(): Failed to reload page: {:?}", error),
                    ))),
            },
            Self::Message::ReceivedId(player_id) => {
                self.client_window.player_id_tag = Some(player_id.0);
                self.client_window
                    .local_storage
                    .set_item(
                        "player_id_tag",
                        &self.client_window.player_id_tag.clone().unwrap(),
                    )
                    .unwrap();
                self.client_window.player_id_key = Some(player_id.1);
                self.client_window
                    .local_storage
                    .set_item(
                        "player_id_key",
                        &self.client_window.player_id_key.clone().unwrap(),
                    )
                    .unwrap();
                false
            }
            Self::Message::Response(client_error) => {
                web_sys::console::log_1(&JsValue::from(format!("{}", client_error)));
                false
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("sky_whole", if self.client_window.day { "sky_day" } else { "sky_night" })}>
                <div class="background">
                    if self.client_window.day {
                        <Clouds max_clouds={self.client_window.animation_level.clone() as usize * 5} day={self.client_window.day} />
                        <div class={classes!("main_screen_ship")}>
                            <img src={format!("{}/extra_files/Menu_Ship_Day.svg", SITE_LINK)} alt={"Ship Riding the Waves"} />
                        </div>
                    } else {
                        <svg width="100%" height="100%">
                            <Clouds max_clouds={self.client_window.animation_level.clone() as usize * 5} day={self.client_window.day} />
                            <Stars max_stars={self.client_window.animation_level.clone() as usize * 100} star_size={2} log={false} />
                        </svg>
                        <div class={classes!("main_screen_ship", "ship_night")}>
                            <img src={format!("{}/extra_files/Menu_Ship_Night.svg", SITE_LINK)} alt={"Ship Riding the Waves"} />
                        </div>
                    }
            </div>
                <Navbar
                    window={self.client_window.window.clone()}
                    day={self.client_window.day}
                    page={self.page_selection.clone()}
                    change_day={ctx.link().callback(move |_| Self::Message::ChangeDayState)}
                    change_page={ctx.link().callback(move |_| Self::Message::ChangePage)} />
                <Panel
                    window={self.client_window.window.clone()}
                    page_selection={self.page_selection.clone()}
                    player_id_tag={self.client_window.player_id_tag.clone().unwrap_or("".to_string())}
                    change_player_id={ctx.link().callback(move |new_player_id: String| Self::Message::ChangePlayerId(new_player_id))}
                    change_animation_level={ctx.link().callback(move |animation_level: AnimationLevel| Self::Message::ChangeAnimationLevel(animation_level))}
                    reload_page={ctx.link().callback(move |_| Self::Message::ReloadPage)}
                    log={false} />
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Menu>::new().render();
}

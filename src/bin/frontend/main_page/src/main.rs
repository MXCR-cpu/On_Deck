use interact::site::SITE_LINK;
use navbar_component::Navbar;
use panel_component::Pages;
use panel_component::Panel;
use utils_files::request::get_request;
use utils_files::sky::Clouds;
use utils_files::sky::Stars;
use utils_files::web_error::ClientError;
use utils_files::window_state::ClientWindow;
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
            }
            Self::Message::ChangePage => {
                self.page_selection = match self.page_selection {
                    Pages::Main => Pages::Settings,
                    Pages::Settings => Pages::Main,
                }
            }
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
            }
            _ => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("sky_whole", if self.client_window.day { "sky_day" } else { "sky_night" })}>
                <div class="background">
                    if self.client_window.day {
                        <Clouds max_clouds={5} day={self.client_window.day} />
                        <div class={classes!("main_screen_ship")}>
                            <img src={format!("{}/extra_files/Menu_Ship_Day.svg", SITE_LINK)} alt={"Ship Riding the Waves"} />
                        </div>
                    } else {
                        <svg width="100%" height="100%">
                            <Clouds max_clouds={5} day={self.client_window.day} />
                            <Stars max_stars={200} star_size={2} log={false} />
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
                    page_selection={self.page_selection.clone()}
                    player_id_tag={self.client_window.player_id_tag.clone().unwrap_or("".to_string())} />
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Menu>::new().render();
}

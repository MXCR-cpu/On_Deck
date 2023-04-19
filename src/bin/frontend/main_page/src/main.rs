use interact::link::GameList;
use interact::link::GameListEntry;
use interact::site::SITE_LINK;
use navbar_component::Navbar;
use utils_files::event_source_state::EventSourceState;
use utils_files::request::{get_request, send_player_amount_update};
use utils_files::sky::Stars;
use utils_files::web_error::ClientError;
use utils_files::window_state::ClientWindow;
use yew::classes;
use yew::html;
use yew::prelude::*;

mod navbar_component;
mod panel_component;

const MAX_PLAYERS: u8 = 8;
const MIN_PLAYERS: u8 = 2;
const DONATION_MESSAGE: &str =
    "Although I am not accepting donations right now, just know that I respect and appreciate your consideration.\n\n\n - MXCR_cpu -";
const GITHUB_LINK: &str = "https://github.com/MXCR-cpu/Battleship";
const INFORMATION: &str =
    "Personal Website as well as explanation of tech stack will be made available in the future";

#[allow(dead_code)]
pub struct Menu {
    client_window: ClientWindow,
    number_of_players: u8,
    links: Option<GameList>,
    settings: bool,
    event_source: EventSourceState,
}

pub enum Pages {
    Main,
    Settings,
}

#[derive(Clone)]
pub enum MenuMsg {
    ChangeDayState,
    AddPlayer,
    SubtractPlayer,
    Settings,
    GoTo(String),
    Alert(String),
    Send(u8),
    Sent,
    NotSending,
    ReceivedId((String, String)),
    NotReceived,
    AwaitUpdate,
    UpdateLinks(Option<Vec<GameListEntry>>),
    EndUpdate,
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
        let callback_update = _ctx
            .link()
            .callback(move |_: ()| Self::Message::AwaitUpdate);
        let callback_end = _ctx.link().callback(move |_: ()| Self::Message::EndUpdate);
        callback_update.emit(());
        let event_source: EventSourceState = EventSourceState::new(
            &format!("{SITE_LINK}/main/page_stream"),
            None,
            move |_| callback_update.emit(()),
            move |_| callback_end.emit(()),
        );
        Self {
            client_window,
            number_of_players: 2,
            links: None,
            settings: false,
            event_source,
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
            Self::Message::AddPlayer => {
                if self.number_of_players < MAX_PLAYERS {
                    self.number_of_players = self.number_of_players + 1;
                }
            }
            Self::Message::SubtractPlayer => {
                if self.number_of_players > MIN_PLAYERS {
                    self.number_of_players = self.number_of_players - 1;
                }
            }
            Self::Message::Settings => {
                self.settings = !self.settings;
            }
            Self::Message::GoTo(hyperlink) => {
                self.client_window
                    .window
                    .location()
                    .set_href(hyperlink.as_str())
                    .unwrap();
            }
            Self::Message::Alert(message) => {
                self.client_window
                    .window
                    .alert_with_message(message.as_str())
                    .unwrap();
            }
            Self::Message::Send(number_of_players) => {
                _ctx.link().send_future(async move {
                    match send_player_amount_update(number_of_players).await {
                        Ok(()) => Self::Message::Sent,
                        Err(error) => Self::Message::Response(
                            error.push(file!(), "update(): failed to send future"),
                        ),
                    }
                });
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
            Self::Message::AwaitUpdate => {
                _ctx.link().send_future(async move {
                    Self::Message::UpdateLinks(
                        get_request::<Option<Vec<GameListEntry>>>(
                            format!("{}/active_game_links", SITE_LINK).as_str(),
                        )
                        .await
                        .unwrap_or(None),
                    )
                });
            }
            Self::Message::UpdateLinks(game_links_option) => {
                self.links = game_links_option;
            }
            Self::Message::EndUpdate => {
                self.event_source.close_connection();
            }
            _ => {}
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let day_callback_value = _ctx.link().send_message(Self::Message::ChangeDayState);
        let day_callback: Callback<MouseEvent> = Callback::from(move |_| day_callback_value);
        let setting_callback_value = _ctx.link().send_message(Self::Message::Settings);
        let setting_callback: Callback<MouseEvent> =
            Callback::from(move |_| setting_callback_value);
        html! {
            <div class={classes!("sky_whole", if self.client_window.day { "sky_day" } else { "sky_night" })}>
                <div class={"background"}>
                    if self.client_window.day {
                        <div class={classes!("main_screen_ship")}>
                            <img src={format!("{}/extra_files/Menu_Ship_Day.svg", SITE_LINK)} alt={"Ship Riding the Waves"} />
                        </div>
                    } else {
                        <Stars max_stars={200} star_size={2} log={false} />
                        <div class={classes!("main_screen_ship", "ship_night")}>
                            <img src={format!("{}/extra_files/Menu_Ship_Night.svg", SITE_LINK)} alt={"Ship Riding the Waves"} />
                        </div>
                    }
                </div>
                <Navbar
                    window={self.client_window.window.clone()}
                    day={self.client_window.day}
                    settings={self.settings}
                    change_day={day_callback}
                    change_setting={setting_callback} />
                <div class={"panel_base"}>{
                    self.render_page(_ctx, if self.settings { Pages::Settings } else { Pages::Main })
                }</div>
            </div>
        }
    }
}

impl Menu {
    fn render_page(&self, _ctx: &Context<Self>, page_type: Pages) -> Html {
        let onclick = |message: MenuMsg| _ctx.link().callback(move |_| message.clone());
        match page_type {
            Pages::Main => {
                html! {
                    <div>
                        <h2 class={classes!("panel_header", "font")}>{ format!("{} Player Free-for-all Battleship", self.number_of_players) }</h2>
                        <div class={classes!("menu_screen", "font")}>
                            <button class={classes!("menu_button", "button_col_0")} onclick={onclick(MenuMsg::AddPlayer)}>{ "Add Player" }</button>
                            <button class={classes!("menu_button", "button_col_1")} onclick={onclick(MenuMsg::Send(self.number_of_players.clone()))}>{ "Start Game" }</button>
                            <button class={classes!("menu_button", "button_col_2")} onclick={onclick(MenuMsg::SubtractPlayer)}>{ "Subtract Player" }</button>
                        </div>
                        <div class={classes!("links_base", "font")}>
                            <ul class={"links_holder"}>
                            {
                                if let Some(player_id_tag) = self.client_window.player_id_tag.clone() {
                                    match &self.links {
                                        Some(item) => item
                                            .iter()
                                            .map(|entry: &GameListEntry| html!{
                                                <li><a class={classes!("links", "font")}
                                                    href={format!("{}/game/{}/{}", SITE_LINK, entry.game_record_number, player_id_tag)}>
                                                    { format!("{:#}", entry) }
                                                    </a>
                                                </li>
                                            })
                                            .collect::<Html>(),
                                        None => html!{ <p class={"font"}>{ "Select the number of players and start the game" }</p> }
                                    }
                                } else {
                                    html! {
                                        <p class={"font"}>{ "Error in Initializing Player Id" }</p>
                                    }
                                }
                            }
                            </ul>
                        </div>
                    </div>
                }
            }
            Pages::Settings => {
                html! {
                    <div>
                        <h2 class={classes!("panel_header", "font")}>{ "Settings" }</h2>
                        <form action="" class={"links_holder"}>
                            <label for="player_id" class={classes!("links", "font")}>{"Player Id:"}</label>
                            <input type="text" id="player_id" name="fname" value={self.client_window.player_id_tag.clone().unwrap()} />
                        </form>
                    </div>
                }
            }
        }
    }
}

fn main() {
    yew::Renderer::<Menu>::new().render();
}

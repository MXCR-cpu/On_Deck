use interact::link::GameList;
use interact::link::GameListEntry;
use interact::site::SITE_LINK;
use std::time::Duration;
use utils_files::request::{get_request, send_player_amount_update};
use utils_files::sky::Sky;
use utils_files::window_state::ClientWindow;
use wasm_bindgen::JsValue;
use yew::classes;
use yew::prelude::*;
use yew::{html, platform::time::sleep};

const MAX_PLAYERS: u8 = 8;
const MIN_PLAYERS: u8 = 2;

const DONATION_MESSAGE: &str =
    "Although I am not accepting donations right now, just know that I respect and appreciate your consideration.\n\n\n - MXCR_cpu -";
const GITHUB_LINK: &str =
    "https://github.com/MXCR-cpu/Battleship";
const INFORMATION: &str =
    "Personal Website as well as explanation of tech stack will be made available in the future";

pub struct Menu {
    client_window: ClientWindow,
    number_of_players: u8,
    links: Option<GameList>,
    settings: bool,
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
    ReceivedId((u32, Vec<u8>)),
    AwaitUpdate,
    UpdateLinks(Vec<GameListEntry>),
    NotReceived,
    None,
}

impl Component for Menu {
    type Message = MenuMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let client_window: ClientWindow = match ClientWindow::new() {
            Ok(window) => window,
            Err(error) => {
                web_sys::console::log_1(&JsValue::from(error));
                panic!()
            }
        };
        if let None = client_window.player_id_tag {
            _ctx.link().send_future(async move {
                match get_request::<(u32, Vec<u8>)>(&format!("{}/get_player_id", SITE_LINK)).await {
                    Ok(result) => Self::Message::ReceivedId(result),
                    Err(error) => {
                        web_sys::console::log_2(
                            &"board_page: create(): get_request(), 112;".into(),
                            &JsValue::from(error),
                        );
                        Self::Message::NotReceived
                    }
                }
            });
        }
        _ctx.link().send_future(async move {
            match get_request::<Vec<GameListEntry>>(
                &format!("{}/active_game_links", SITE_LINK).as_str(),
            )
            .await
            {
                Ok(result) => Self::Message::UpdateLinks(result),
                Err(error) => {
                    web_sys::console::log_2(
                        &"board_page: create(): get_request(), 112;".into(),
                        &JsValue::from(error),
                    );
                    Self::Message::NotReceived
                }
            }
        });
        Self {
            client_window,
            number_of_players: 2,
            links: None,
            settings: false,
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
                            "true"
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
                        Err(_) => Self::Message::NotSending,
                    }
                });
            }
            Self::Message::ReceivedId(player_id) => {
                // web_log(format!("battleship: new player_id: {}", player_id));
                self.client_window.player_id_tag = Some(format!("player_no.{}", player_id.0));
                self.client_window
                    .local_storage
                    .set_item(
                        "player_id",
                        &self.client_window.player_id_tag.clone().unwrap(),
                    )
                    .unwrap();
                self.client_window.player_id_key = Some(format!("{:?}", player_id.1));
                self.client_window
                    .local_storage
                    .set_item(
                        "player_key",
                        &self.client_window.player_id_key.clone().unwrap(),
                    )
                    .unwrap();
            }
            Self::Message::AwaitUpdate => {
                _ctx.link().send_future(async move {
                    sleep(Duration::from_secs(10)).await;
                    Self::Message::UpdateLinks(
                        get_request::<Vec<GameListEntry>>(
                            format!("{}/active_game_links", SITE_LINK).as_str(),
                        )
                        .await
                        .unwrap(),
                    )
                });
            }
            Self::Message::UpdateLinks(game_links) => {
                self.links = Some(game_links);
                _ctx.link().send_message(Self::Message::AwaitUpdate);
            }
            _ => {}
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let onclick = |message: Self::Message| _ctx.link().callback(move |_| message.clone());
        html! {
            <div class={classes!("sky_whole", if self.client_window.day { "sky_day" } else { "sky_night" })}>
                <div class={"background"}>
                    if self.client_window.day {
                        <div class={classes!("main_screen_ship")}>
                            <img src={format!("{}/extra_files/Menu_Ship_Day.svg", SITE_LINK)} alt={"Ship Riding the Waves"} />
                        </div>
                    } else {
                        <Sky max_stars={200} star_size={2} log={false} />
                        <div class={classes!("main_screen_ship", "ship_night")}>
                            <img src={format!("{}/extra_files/Menu_Ship_Night.svg", SITE_LINK)} alt={"Ship Riding the Waves"} />
                        </div>
                    }
                </div>
                <div class={"top_row"}>
                    <button class={"button_col_0"} onclick={onclick(Self::Message::Alert(DONATION_MESSAGE.to_string()))} alt={"Donations"}>
                        { "💸" }
                    </button>
                    <button class={"button_col_1"} onclick={onclick(Self::Message::GoTo(GITHUB_LINK.to_string()))}>
                        { "🐙" }
                    </button>
                    <button class={"button_col_3"} onclick={onclick(Self::Message::Alert(INFORMATION.to_string()))}>
                        { "🧠" }
                    </button>
                    <button class={"button_col_4"} onclick={onclick(Self::Message::ChangeDayState)}>{
                        if self.client_window.day { "☀️" } else { "🌙" }
                    }</button>
                    <button class={"button_col_5"} onclick={onclick(Self::Message::Settings)}>{
                        if self.settings { "🚀" } else { "⚙️" }
                    }</button>
                </div>
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
                                        <p class={"font"}>{ "Player Id Tag Loading ..." }</p>
                                    }
                                }
                            }
                            </ul>
                        </div>
                    </div>
                }
            }
            // This needs to be completed
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

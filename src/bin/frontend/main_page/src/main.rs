// use link::Links;
// use crate::request::get_request;
use crate::web_error::ErrorTrait;
use indexmap::IndexMap;
use js_sys::Array;
use js_sys::Function;
use wasm_bindgen::JsValue;
use web_sys::EventSource;
use web_sys::{Storage, Window};
use yew::classes;
use yew::prelude::*;
use yew::{
    html,
    virtual_dom::{ApplyAttributeAs, Attributes},
    AttrValue,
};

mod request;
mod web_error;

const MAX_PLAYERS: u8 = 8;
const MIN_PLAYERS: u8 = 2;
const SITE_LINK: &str = "http://127.0.0.1:8000";
const DONATION_MESSAGE: &str = "Although I am not accepting donations right now, just know that I respect and appreciate your consideration.\n\n\n - MXCR_cpu -";
const GITHUB_LINK: &str = "https://github.com/MXCR-cpu/Battleship";
const INFORMATION: &str =
    "Personal Website as well as explanation of tech stack will be made available in the future";
//stream_links is the argument
const JAVASCRIPT_EVENTSOURCE_BODY: &str =
"
    // if stream_links.data[0] == 'S' {
    //     return;
    // }
    const stream = JSON.parse(stream_links.data);
    console.log(stream);
    document.getElementsByTagName('ul')[0].replaceChildren();

    for(var i = 0; i < stream.length; i++) {
        document.getElementsByTagName('ul')[0].innerHTML +=
            `<li class=\"links font\">
                <a href=\"http://127.0.0.1:8000/game/${stream[i]}/${window.localStorage['player_id']}\">
                    Game ${stream[i]}
                </a>
            </li>`;
    }
";

#[allow(dead_code)]
pub struct Menu {
    number_of_players: u8,
    links: Option<Attributes>,
    day: bool,
    settings: bool,
    player_id: String,
    window: Window,
    storage: Storage,
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
    ReceivedLinks(Vec<String>),
    ReceivedId(String),
    NotReceived,
    None,
}

impl Component for Menu {
    type Message = MenuMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let window: Window = match web_sys::window() {
            Some(window) => window,
            None => "battleship: Window object not found"
                .to_string()
                .log_error(),
        };
        let storage: Storage = match window.local_storage() {
            Ok(option) => match option {
                Some(storage) => storage,
                None => "battleship: Local Storage Object not found"
                    .to_string()
                    .log_error(),
            },
            Err(error) => error.log_error(),
        };
        let mut player_id: String = String::new();
        match storage.get_item("player_id") {
            Ok(value) => match value {
                Some(inner_player_id) => {
                    player_id = inner_player_id.to_string();
                }
                None => {
                    web_sys::console::log(&Array::from(&JsValue::from(
                        "battleship: player_id not found, retrieving new player_id...",
                    )));
                    _ctx.link().send_future(async move {
                        match request::get_request::<u32>(
                            format!("{}/get_player_id", SITE_LINK).as_str(),
                        )
                        .await
                        {
                            Ok(player_id) => Self::Message::ReceivedId(player_id.to_string()),
                            Err(_) => Self::Message::NotReceived,
                        }
                    });
                }
            },
            Err(error) => error.log_error(),
        }
        EventSource::new(format!("{}/stream", SITE_LINK).as_str())
            .unwrap()
            .set_onmessage(Some(&Function::new_with_args(
                "stream_links",
                JAVASCRIPT_EVENTSOURCE_BODY,
            )));
        Self {
            number_of_players: 2,
            links: None,
            day: true,
            settings: false,
            player_id,
            window,
            storage,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Self::Message::ChangeDayState => {
                self.day = !self.day;
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
                self.window.location().set_href(hyperlink.as_str()).unwrap();
            }
            Self::Message::Alert(message) => {
                self.window.alert_with_message(message.as_str()).unwrap();
            }
            Self::Message::Send(number_of_players) => {
                _ctx.link().send_future(async move {
                    match request::send_player_amount_update(number_of_players).await {
                        Ok(()) => Self::Message::Sent,
                        Err(_) => Self::Message::NotSending,
                    }
                });
            }
            Self::Message::Sent => {
                _ctx.link().send_future(async move {
                    match request::get_request::<Vec<String>>(
                        format!("{}/stream", SITE_LINK).as_str(),
                    )
                    .await
                    {
                        Ok(links) => Self::Message::ReceivedLinks(links),
                        Err(_) => Self::Message::NotReceived,
                    }
                });
            }
            Self::Message::ReceivedId(player_id) => {
                web_error::web_log(format!("battleship: new player_id: {}", player_id));
                self.player_id = player_id;
                self.storage.set_item("player_id", &self.player_id).unwrap();
            }
            Self::Message::ReceivedLinks(links) => {
                web_error::web_log("battleship: received game links".to_string());
                let mut links_index_array: IndexMap<AttrValue, (AttrValue, ApplyAttributeAs)> =
                    IndexMap::new();
                let mut index: i8 = 0;
                for link in links.into_iter() {
                    *links_index_array
                        .entry(AttrValue::from(index.to_string()))
                        .or_insert((
                            AttrValue::from("".to_string()),
                            ApplyAttributeAs::Attribute,
                        )) = (
                        AttrValue::from(format!("{}", link)),
                        ApplyAttributeAs::Attribute,
                    );
                    index += 1;
                }
                self.links = Some(Attributes::IndexMap(links_index_array));
            }
            _ => {}
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let onclick = |message: Self::Message| _ctx.link().callback(move |_| message.clone());
        html! {
            <div>
                <style>{
                    if self.day {
                        "html {
                            background-color: var(--normal_sky_color);
                            transition: background-color 0.5s ease;
                        }"
                    } else {
                        "html {
                            background-color: var(--night_sky_color);
                            transition: background-color 0.5s ease;
                        }"
                    }
                }</style>
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
                        if self.day { "☀️" } else { "🌙" }
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
                            // {
                                // match &self.links {
                                //     Some(item) => item
                                //         .iter()
                                //         .map(|(_key, value): (&str, &str)| html!{
                                //             <li><a class={classes!("links", "font")}
                                //                 href={format!("{}/game/{}/{}", SITE_LINK, value, self.player_id)}>
                                //                 { format!("Game {}", value) }
                                //                 </a>
                                //             </li>
                                //         })
                                //         .collect::<Html>(),
                                //     None => html!{ <p class={"font"}>{ "Select the number of players and start the game" }</p> }
                                // }
                            // }
                            </ul>
                        </div>
                    </div>
                }
            }
            Pages::Settings => {
                html! {
                    <div>
                        <h2 class={classes!("panel_header", "font")}>{ "Settings" }</h2>
                    </div>
                }
            }
        }
    }
}

fn main() {
    yew::Renderer::<Menu>::new().render();
}

// use link::Links;
use crate::web_error::ErrorTrait;
use indexmap::IndexMap;
use js_sys::Array;
use wasm_bindgen::JsValue;
use web_sys::{Storage, Window};
use yew::classes;
use yew::prelude::*;
use yew::{
    html,
    virtual_dom::{ApplyAttributeAs, Attributes},
    AttrValue,
};

mod web_error;
mod request;

const MAX_PLAYERS: u8 = 8;
const MIN_PLAYERS: u8 = 2;

#[allow(dead_code)]
pub struct Menu {
    number_of_players: u8,
    links: Option<Attributes>,
    day: bool,
    player_id: String,
    window: Window,
    storage: Storage,
}

#[derive(Clone)]
pub enum MenuMsg {
    ChangeDayState,
    AddPlayer,
    SubtractPlayer,
    GoTo(String),
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
                        match request::get_request::<u32>("http://127.0.0.1:8000/get_player_id").await {
                            Ok(player_id) => Self::Message::ReceivedId(player_id.to_string()),
                            Err(_) => Self::Message::NotReceived,
                        }
                    });
                }
            },
            Err(error) => error.log_error(),
        }
        Self {
            number_of_players: 2,
            links: None,
            day: true,
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
            Self::Message::GoTo(hyperlink) => {
                self.window.location().set_href(hyperlink.as_str()).unwrap();
            }
            Self::Message::Send(number_of_players) => {
                _ctx.link().send_future(async move {
                    match request::send_player_amount_update(number_of_players).await {
                        Ok(()) => Self::Message::Sent,
                        Err(_) => Self::Message::NotSending,
                    }
                });
                _ctx.link().send_message(Self::Message::Sent);
            }
            Self::Message::Sent => {
                _ctx.link().send_future(async move {
                    match request::get_request::<Vec<String>>("http://127.0.0.1:8000/game_links").await {
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
                    <button class={classes!("button_col_0")} onclick={onclick(Self::Message::GoTo("https://github.com/MXCR-cpu/Battleship".to_string()))}>
                        { "🐙" }
                    </button>
                    <button class={classes!("button_col_1")} onclick={onclick(Self::Message::ChangeDayState)}>{
                        if self.day { "☀️" } else { "🌙" }
                    }</button>
                </div>
                <div class={"panel_base"}>
                    <h2 class={"font_header"} style={"font-size: 36px;"}>{ format!("{} Player Free-for-all Battleship", self.number_of_players) }</h2>
                    <div class={classes!("menu_screen", "font")}>
                        <button class={classes!("menu_button", "button_col_0")} onclick={onclick(Self::Message::AddPlayer)}>{ "Add Player" }</button>
                        <button class={classes!("menu_button", "button_col_1")} onclick={onclick(Self::Message::Send(self.number_of_players.clone()))}>{ "Start Game" }</button>
                        <button class={classes!("menu_button", "button_col_2")} onclick={onclick(Self::Message::SubtractPlayer)}>{ "Subtract Player" }</button>
                    </div>
                    <div class={"links_base"}>
                        <ul>{
                            match &self.links {
                                Some(item) => item
                                    .iter()
                                    .map(|(_key, value): (&str, &str)| html!{ <p class={"font"}><a href={format!("{}/{}", value, self.player_id)}>{ "Game" }</a></p> })
                                    .collect::<Html>(),
                                None => html!{ <p class={"font"}>{ "Select the number of players and start the game" }</p> }
                            }
                        }</ul>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Menu>::new().render();
}

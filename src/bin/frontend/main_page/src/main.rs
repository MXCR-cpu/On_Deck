// use link::Links;
use indexmap::IndexMap;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use yew::classes;
use yew::prelude::*;
use yew::{
    html,
    virtual_dom::{ApplyAttributeAs, Attributes},
    AttrValue,
};
use state::Storage;

const MAX_PLAYERS: u8 = 8;
const MIN_PLAYERS: u8 = 2;
static GLOBAL_MAP: Storage<&'static str> = Storage::new();

// #[derive(Serialize, Deserialize, Clone)]
// pub struct Links {
//     hyperlinks: Vec<String>,
// }

pub struct Menu {
    number_of_players: u8,
    links: Option<Attributes>,
    day: bool,
    player_id: u32,
}

#[derive(Clone)]
pub enum MenuMsg {
    ChangeDayState,
    AddPlayer,
    SubtractPlayer,
    Send(u8),
    Sent,
    NotSending,
    ReceivedLinks(Vec<String>),
    ReceivedId(u32),
    NotReceived,
    None,
}

impl Component for Menu {
    type Message = MenuMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        _ctx
            .link()
            .send_future(async move {
                match get_request::<u32>("http://127.0.0.1:8000/get_player_id").await {
                    Ok(player_id) => Self::Message::ReceivedId(player_id),
                    Err(_) => Self::Message::NotReceived,
                }
            });
        Self {
            number_of_players: 2,
            links: None,
            day: true,
            player_id: 0,
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
            Self::Message::Send(number_of_players) => {
                _ctx.link().send_future(async move {
                    match send_player_amount_update(number_of_players).await {
                        Ok(()) => Self::Message::Sent,
                        Err(_) => Self::Message::NotSending,
                    }
                });
                _ctx.link().send_message(Self::Message::Sent);
            }
            Self::Message::Sent => {
                _ctx.link().send_future(async move {
                    match get_request::<Vec<String>>("http://127.0.0.1:8000/game_links").await {
                        Ok(links) => Self::Message::ReceivedLinks(links),
                        Err(_) => Self::Message::NotReceived,
                    }
                });
            }
            Self::Message::ReceivedId(player_id) => {
                self.player_id = player_id;
            }
            Self::Message::ReceivedLinks(links) => {
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
                <button class={"change_day_state_button"} onclick={onclick(Self::Message::ChangeDayState)}>{
                    if self.day {
                        "☀️"
                    } else {
                        "🌙"
                    }
                }</button>
                <div class={"panel_base"}>
                    <h2 class={"font_header"} style={"font-size: 36px;"}>{ format!("{} Player Free-for-all Battleship", self.number_of_players) }</h2>
                    <div class={classes!("menu_screen", "font")}>
                        <button class={classes!("menu_button", "button_col_0")} onclick={onclick(Self::Message::AddPlayer)}>{ "Add Player" }</button>
                        <button class={classes!("menu_button", "button_col_2")} onclick={onclick(Self::Message::Send(self.number_of_players.clone()))}>{ "Start Game" }</button>
                        <button class={classes!("menu_button", "button_col_1")} onclick={onclick(Self::Message::SubtractPlayer)}>{ "Subtract Player" }</button>
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

async fn get_request<T: DeserializeOwned>(link: &str) -> Result<T, reqwest::Error> {
    Ok(reqwest::Client::new()
       .get(link)
       .send()
       .await?
       .json::<T>()
       .await?)
}

async fn send_player_amount_update<'a>(number_of_players: u8) -> Result<(), reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("number_of_players".to_string(), number_of_players.clone());
    let mut _result: HashMap<String, Vec<String>> = HashMap::new();
    reqwest::Client::new()
        .post("http://127.0.0.1:8000/start")
        .json::<HashMap<String, u8>>(&map)
        .send()
        .await?;
    Ok(())
}

fn main() {
    yew::Renderer::<Menu>::new().render();
}

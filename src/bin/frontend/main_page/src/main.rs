use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yew::classes;
use yew::prelude::*;
use yew::{
    html,
    virtual_dom::{ApplyAttributeAs, Attributes},
    AttrValue,
};

const MAX_PLAYERS: i8 = 8;
const MIN_PLAYERS: i8 = 2;

#[derive(Serialize, Deserialize, Clone)]
pub struct Links {
    hyperlinks: Vec<String>,
}

pub struct Menu {
    number_of_players: i8,
    links: Option<Attributes>,
}

#[derive(Clone)]
pub enum MenuMsg {
    AddPlayer,
    SubtractPlayer,
    Send(i8),
    Sent,
    NotSending,
    Receive(Links),
    NotReceived,
    None,
}

impl Component for Menu {
    type Message = MenuMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            number_of_players: 2,
            links: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
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
                    match get_current_links_information().await {
                        Ok(links) => Self::Message::Receive(links),
                        Err(_) => Self::Message::NotReceived,
                    }
                });
            }
            Self::Message::Receive(links) => {
                let mut links_index_array: IndexMap<AttrValue, (AttrValue, ApplyAttributeAs)> =
                    IndexMap::new();
                let mut index: i8 = 0;
                for link in links.hyperlinks.into_iter() {
                    *links_index_array
                        .entry(AttrValue::from(index.to_string()))
                        .or_insert((
                            AttrValue::from("".to_string()),
                            ApplyAttributeAs::Attribute,
                        )) = (AttrValue::from(link), ApplyAttributeAs::Attribute);
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
            <div classes={"base_page"} style={"background-color: lightblue;"}>
                <h2 class={"font"}>{ format!("{} Player Battleship", self.number_of_players) }</h2>
                <div classes={classes!("menu_screen", "font")}>
                    <button class={"menu_button"} onclick={onclick(Self::Message::AddPlayer)}>{ "Add Player" }</button>
                    <button class={"menu_button"} onclick={onclick(Self::Message::SubtractPlayer)}>{ "Subtract Player" }</button>
                    <button class={"menu_button"} onclick={onclick(Self::Message::Send(self.number_of_players.clone()))}>{ "Start Game" }</button>
                </div>
                <div>
                    <ul>{
                        match &self.links {
                            Some(item) => item
                                .iter()
                                .map(|(_key, value): (&str, &str)| html!{ <p class={"font"}><a href={value.to_string()}>{ value.to_string() }</a></p> })
                                .collect::<Html>(),
                            None => html!{ <p class={"font"}>{ "Select the number of players and start the game" }</p> }
                        }
                    }</ul>
                </div>
            </div>
        }
    }
}

async fn send_player_amount_update<'a>(number_of_players: i8) -> Result<(), reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("number_of_players".to_string(), number_of_players.clone());
    let mut _result: HashMap<String, Vec<String>> = HashMap::new();
    reqwest::Client::new()
        .post("http://127.0.0.1:8000/start")
        .json::<HashMap<String, i8>>(&map)
        .send()
        .await?;
    Ok(())
}

async fn get_current_links_information() -> Result<Links, serde_json::Error> {
    let json_links: Result<String, reqwest::Error> = reqwest::Client::new()
        .get("http://127.0.0.1:8000/links")
        .send()
        .await
        .unwrap()
        .json::<String>()
        .await;
    serde_json::from_str(json_links.unwrap().as_str())
}

fn main() {
    yew::Renderer::<Menu>::new().render();
}

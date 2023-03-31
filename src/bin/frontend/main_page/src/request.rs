use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub async fn get_request<T: DeserializeOwned>(link: &str) -> Result<T, reqwest::Error> {
    Ok(reqwest::Client::new()
        .get(link)
        .send()
        .await?
        .json::<T>()
        .await?)
}

pub async fn send_player_amount_update<'a>(number_of_players: u8) -> Result<(), reqwest::Error> {
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

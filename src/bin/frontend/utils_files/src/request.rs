use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

use crate::web_error::ClientError;

pub async fn get_request<T: DeserializeOwned>(link: &str) -> Result<T, ClientError> {
    reqwest::Client::new()
        .get(link)
        .send()
        .await
        .map_err(|error: _| ClientError::from(file!(), &format!("get_request(): reqwest failed to send client get request: {}", error)))?
        .json::<T>()
        .await
        .map_err(|error: _| ClientError::from(file!(), &format!("get_request(): reqwest failed to parse json get request to respective type: {}", error)))
}

pub async fn send_player_amount_update<'a>(number_of_players: u8) -> Result<(), ClientError> {
    let mut map = HashMap::new();
    map.insert("number_of_players".to_string(), number_of_players.clone());
    reqwest::Client::new()
        .post("http://127.0.0.1:8000/start")
        .json::<HashMap<String, u8>>(&map)
        .send()
        .await
        .map(|_| ())
        .map_err(|error: _| {
            ClientError::from(
                file!(),
                &format!(
                    "send_player_amount_update(): failed to send client post request: {}",
                    error
                ),
            )
        })
}

pub async fn fire_on_position<T: DeserializeOwned + Serialize>(
    item: T,
    game_number: u32,
) -> Result<(), ClientError> {
    reqwest::Client::new()
        .post(format!("http://127.0.0.1:8000/game/fire/{}", game_number))
        .json::<T>(&item)
        .send()
        .await
        .map(|_| ())
        .map_err(|error: _| {
            ClientError::from(
                file!(),
                &format!(
                    "fire_on_position(): Failed to Send Fire Post Request: {}",
                    error
                ),
            )
        })
}

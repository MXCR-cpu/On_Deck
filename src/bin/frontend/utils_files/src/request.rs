use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

pub async fn get_request<T: DeserializeOwned>(link: &str) -> Result<T, String> {
    match reqwest::Client::new()
        .get(link)
        .send()
        .await {
            Ok(result_outer) => match result_outer.json::<T>()
                .await {
                    Ok(result_inner) => Ok(result_inner),
                    Err(error) => { 
                        Err(format!("request.rs: get_request(): reqwest Failed to parse Json Get Request To Respective Type; \n\t\t{}", error))
                    }
                },
            Err(error) => Err(format!("request.rs, 6: reqwest Failed to Send Client Get Request; \n\t\t{}", error))
        }
}

pub async fn send_player_amount_update<'a>(number_of_players: u8) -> Result<(), String> {
    let mut map = HashMap::new();
    map.insert("number_of_players".to_string(), number_of_players.clone());
    let mut _result: HashMap<String, Vec<String>> = HashMap::new();
    match reqwest::Client::new()
        .post("http://127.0.0.1:8000/start")
        .json::<HashMap<String, u8>>(&map)
        .send()
        .await {
            Ok(_) => Ok(()),
            Err(error) => Err(format!("request.rs: send_player_amount_update(): Failed to Send Client Post Request; \n\t\t{}", error))
        }
}

pub async fn fire_on_position<T: DeserializeOwned + Serialize>(item: T, game_number: u32) -> Result<(), String> {
    match reqwest::Client::new()
        .post(format!("http://127.0.0.1:8000/fire/{}", game_number))
        .json::<T>(&item)
        .send()
        .await {
            Ok(_) => Ok(()),
            Err(error) => Err(format!("request.rs: fire_on_position(): Failed to Send Fire Post Request; \n\t\t{}", error))
        }
}

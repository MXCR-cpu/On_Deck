use crate::board::Board;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub boards: Board,
    pub number_of_players: usize,
    pub player_tags: Vec<String>,
    pub challenge: String,
    pub game_number: u64,
    pub shot_list: u32,
}

impl Game {
    pub fn new(number_of_players: usize, game_number: u64) -> Self {
        Self {
            boards: Board::new(number_of_players),
            number_of_players,
            player_tags: Vec::with_capacity(number_of_players),
            challenge: String::new(),
            game_number,
            shot_list: 0,
        }
    }

    pub fn get_link(&self) -> String {
        format!("http://127.0.0.1:8000/game/{}", self.game_number)
    }
}

impl From<&Game> for String {
    fn from(game: &Game) -> Self {
        serde_json::to_string(game).unwrap()
    }
}

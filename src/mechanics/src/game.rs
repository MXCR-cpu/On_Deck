use crate::board::Board;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Game {
    boards: Board,
    number_of_players: usize,
    player_tags: Vec<String>,
    challenge: String,
    game_number: u64,
}

impl Game {
    pub fn new(number_of_players: usize, game_number: u64) -> Self {
        Self {
            boards: Board::new(number_of_players),
            number_of_players,
            player_tags: Vec::with_capacity(number_of_players),
            challenge: String::new(),
            game_number,
        }
    }

    pub fn get_link(&self) -> String {
        format!("http://127.0.0.1:8000/game/{}", self.game_number)
    }
}

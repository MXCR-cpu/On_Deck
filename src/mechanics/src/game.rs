use crate::board::Board;
use serde::{Serialize, Deserialize};

// pub type Boards = Vec<Board>;
// pub type Links = Vec<u32>;

#[derive(Serialize, Deserialize)]
pub struct Game {
    boards: Board,
    number_of_players: usize,
    player_tags: Vec<String>,
    challenges: Vec<Vec<u8>>,
    game_number: u64,
}

impl Game {
    pub fn new(number_of_players: usize, game_number: u64) -> Self {
        Self {
            boards: Board::new(number_of_players),
            number_of_players,
            player_tags: Vec::with_capacity(number_of_players),
            challenges: Vec::with_capacity(number_of_players),
            game_number,
        }
    }

    pub fn get_link(&self) -> String {
        format!("http://127.0.0.1:8000/game/{}", self.game_number)
    }
}

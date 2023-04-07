use crate::board::Board;
use serde::{Serialize, Deserialize};
use rand::prelude::*;

// pub type Boards = Vec<Board>;
pub type Links = Vec<u32>;

#[derive(Serialize, Deserialize)]
pub struct Game {
    boards: Board,
    player_links: Links,
    game_number: u64,
}

impl Game {
    pub fn new(number_of_players: usize, game_number: u64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            boards: Board::new(number_of_players),
            player_links: (0..number_of_players)
                .map(|_| {
                    rng.gen::<u32>()
                })
                .collect::<Links>(),
            game_number,
        }
    }

    pub fn get_link(&self) -> String {
        format!("http://127.0.0.1:8000/game/{}", self.game_number)
    }
}

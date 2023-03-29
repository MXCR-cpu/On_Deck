use crate::board::Board;
use crate::link::Links;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Game {
    boards: Vec<Board>,
    player_links: Links,
    game_link: String,
}

impl Game {
    pub fn new(number_of_players: u8) -> Self {
        Self {
            boards: (0..number_of_players).map(|_| Board::new()).collect::<Vec<Board>>(),
            player_links: Links::new(number_of_players),
            game_link: format!("http://127.0.0.1:8000/game/{}", number_of_players),
        }
    }
}

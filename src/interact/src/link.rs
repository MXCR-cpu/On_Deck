use serde::{Serialize, Deserialize};
use std::fmt;

pub type GameList = Vec<GameListEntry>;

#[derive(Serialize, Deserialize, Clone)]
pub struct GameListEntry {
    pub game_record_number: u64,
    total_positions: usize,
    active_player_names: Vec<String>,
}

impl GameListEntry {
    pub fn new(game_record_number: u64, total_positions: usize) -> Self {
        Self {
            game_record_number,
            total_positions,
            active_player_names: Vec::new(),
        }
    }

    pub fn add_player(&self, player_name: String) -> Result<Self, &str> {
        if self.is_full() {
            return Err("Game is already full");
        }
        let mut player_list: Vec<String> = self.active_player_names.clone();
        player_list.push(player_name);
        Ok(Self {
            game_record_number: self.game_record_number,
            total_positions: self.total_positions,
            active_player_names: player_list,
        })
    }

    fn is_full(&self) -> bool {
        self.active_player_names.len() == self.total_positions
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl fmt::Display for GameListEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let taken_positions: u8 = self.active_player_names.len() as u8;
        formatter.write_fmt(format_args!(
            "Game {:0>3}   {}/{}   {:?}",
            self.game_record_number,
            taken_positions,
            self.total_positions,
            self.active_player_names
        ))
    }
}


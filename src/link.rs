use serde::{Deserialize, Serialize};
use rand::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Links {
    hyperlinks: Vec<u32>,
}

impl Links {
    pub fn new(number_of_players: u8) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            hyperlinks: (0..number_of_players)
                .map(|_| (rng.gen::<f64>() * 1000.0) as u32)
                .collect::<Vec<u32>>(),
        }
    }

    pub fn get_links(&self) -> Vec<u32> {
        self.hyperlinks.clone()
    }
}

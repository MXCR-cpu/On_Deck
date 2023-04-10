use crate::position::{FiredState, Position};
use crate::ship::{Ship, ShipSet};
use serde::{Deserialize, Serialize};

pub type PositionVectors = Vec<Vec<Position>>;

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub board: PositionVectors,
    ship_set: Vec<Vec<Ship>>,
}

impl Board {
    pub fn new(players: usize) -> Self {
        Self {
            board: Self::initialize_board(players),
            ship_set: ShipSet::new(players),
        }
    }

    pub fn initialize_board(players: usize) -> PositionVectors {
        (0..10)
            .map(|index| {
                (0..10)
                    .map(|jndex| Position::new(index, jndex, None, players).unwrap())
                    .collect::<Vec<Position>>()
            })
            .collect::<PositionVectors>()
    }

    pub fn empty() -> Self {
        Self {
            board: Self::initialize_board(0),
            ship_set: ShipSet::new(0),
        }
    }

    pub fn fire(&self, lat: usize, lon: usize, to: usize) -> Result<PositionVectors, String> {
        let mut new_fired_state: Vec<FiredState> = self.board[lat][lon].get_fired_state();
        if to <= new_fired_state.len() as usize {
            return Err(format!(
                "board: fire: {} is less than 0 or greater/equal to the length of the player list ({})",
                to,
                new_fired_state.len()));
        }
        let mut new_board = self.board.clone();
        new_fired_state[to - 1] = if self.ship_set[to - 1]
            .clone()
            .into_iter()
            .fold(false, |acc: bool, ship: Ship| {
                acc || ship.check_hit(lat, lon)
            }) {
            FiredState::Hit
        } else {
            FiredState::Miss
        };
        new_board[lat as usize][lon as usize] = Position::new(lat, lon, Some(new_fired_state), 0)?;
        Ok(new_board)
    }
}

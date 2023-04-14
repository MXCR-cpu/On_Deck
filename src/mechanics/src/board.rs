use crate::position::{FiredState, Position};
use crate::ship::{Ship, ShipSet};
use serde::{Deserialize, Serialize};

const BOARD_SIZE: usize = 10;
pub type PositionVectors = Vec<Vec<Position>>;

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub positions: PositionVectors,
    pub ship_set: Vec<Vec<Ship>>,
    players: usize,
}

impl Board {
    pub fn new(players: usize) -> Self {
        Self {
            positions: Self::initialize_board(players),
            ship_set: ShipSet::new(players),
            players,
        }
    }

    pub fn initialize_board(players: usize) -> PositionVectors {
        (0..BOARD_SIZE)
            .map(|index| {
                (0..BOARD_SIZE)
                    .map(|jndex| Position::new(index, jndex, None, players).unwrap())
                    .collect::<Vec<Position>>()
            })
            .collect::<PositionVectors>()
    }

    pub fn empty() -> Self {
        Self {
            positions: Self::initialize_board(0),
            ship_set: ShipSet::new(0),
            players: 0,
        }
    }

    pub fn start_board(&mut self) {
        self.positions = (0..BOARD_SIZE)
            .map(|x_index: usize| {
                (0..BOARD_SIZE)
                    .map(|y_index: usize| {
                        Position::new(
                            x_index,
                            y_index,
                            Some(vec![FiredState::Untouched; self.players]),
                            self.players,
                        )
                        .unwrap()
                    })
                    .collect::<Vec<Position>>()
            })
            .collect::<PositionVectors>()
    }

    pub fn get_board_with_player_positions(&self, player_index: usize) -> PositionVectors {
        let player_ships: Vec<(usize, usize)> = self.ship_set[player_index]
            .iter()
            .map(|ship: &Ship| ship.location.clone())
            .collect::<Vec<Vec<(usize, usize)>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<(usize, usize)>>();
        let mut player_personal_board: PositionVectors = self.positions.clone();
        for (x_index, y_index) in player_ships.iter() {
            if player_personal_board[*x_index][*y_index].fired_state[player_index]
                != FiredState::Hit
            {
                player_personal_board[*x_index][*y_index].fired_state[player_index] =
                    FiredState::Ship;
            }
        }
        player_personal_board
    }

    pub fn fire(&mut self, lon: usize, lat: usize, to: usize) -> Result<PositionVectors, String> {
        let mut new_fired_state: Vec<FiredState> = self.positions[lon][lat].get_fired_state();
        if to > new_fired_state.len() - 1 as usize {
            return Err(format!(
                "board: fire: {} is greater than the length of the player list index ({})",
                to,
                new_fired_state.len() - 1
            ));
        }
        let mut new_positions = self.positions.clone();
        new_fired_state[to] = if self.ship_set[to]
            .clone()
            .into_iter()
            .fold(false, |acc: bool, ship: Ship| {
                acc || ship.check_hit(lon, lat)
            }) {
            FiredState::Hit
        } else {
            FiredState::Miss
        };
        new_positions[lon as usize][lat as usize] = Position::new(lon, lat, Some(new_fired_state), 0)?;
        Ok(new_positions)
    }
}

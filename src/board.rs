use crate::position::Position;
use crate::ship::Ship;
use rocket::serde::{Deserialize, Serialize};

type BoardTop = Vec<Vec<Position>>;
type ShipSet = Vec<Ship>;

#[derive(Serialize, Deserialize)]
pub struct Board {
    board: BoardTop,
    ships: ShipSet,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: Self::initialize_board(),
            ships: Ship::new_ships(),
        }
    }

    pub fn initialize_board() -> BoardTop {
        (0..10)
            .map(|index| {
                (0..10)
                    .map(|jndex| Position::new(index, jndex, None).unwrap())
                    .collect::<Vec<Position>>()
            })
            .collect::<BoardTop>()
    }

    pub fn fire(&self, lat: i8, lon: i8, to: i8) -> Result<BoardTop, String> {
        let mut new_fired_state = self.board[lat as usize][lon as usize].get_fired_state();
        if to < 0 || to <= new_fired_state.len() as i8 {
            return Err(format!(
                "board: fire: {} is less than 0 or greater/equal to the length of the player list ({})",
                to,
                new_fired_state.len()));
        }
        let mut new_board = self.board.clone();
        new_fired_state[(to - 1) as usize] = true;
        new_board[lat as usize][lon as usize] = Position::new(lat, lon, Some(new_fired_state))?;
        Ok(new_board)
    }
}

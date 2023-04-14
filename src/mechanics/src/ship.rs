use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Ship {
    pub name: String,
    pub location: Vec<(usize, usize)>,
}

#[allow(dead_code)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Ship {
    pub fn new(ship_name: String, pos: Result<Vec<(usize, usize)>, String>) -> Self {
        match pos {
            Ok(p) => Self {
                name: ship_name,
                location: p,
            },
            Err(e) => panic!("{}: {}", ship_name, e),
        }
    }

    pub fn new_ships() -> Vec<Self> {
        [
            ("Carrier".to_string(), 5),
            ("Battleship".to_string(), 4),
            ("Destroyer".to_string(), 3),
            ("Submarine".to_string(), 3),
            ("Patrol Boat".to_string(), 2),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, (name, size))| {
            Self::new(name, Self::direction((0, index), size, Direction::East))
        })
        .collect::<Vec<Self>>() // make sure to include ship collision detection
    }

    fn direction(
        pos: (usize, usize),
        size: usize,
        direction: Direction,
    ) -> Result<Vec<(usize, usize)>, String> {
        Ok((0..size)
            .map(|index: usize| {
                (
                    match direction {
                        Direction::East => pos.0 + index,
                        Direction::West => pos.0 - index,
                        _ => pos.0,
                    },
                    match direction {
                        Direction::South => pos.1 - index,
                        Direction::North => pos.1 + index,
                        _ => pos.1,
                    },
                )
            })
            .collect::<Vec<(usize, usize)>>())
    }

    pub fn check_hit(&self, index: usize, jndex: usize) -> bool {
        self.location
            .iter()
            .fold(false, |acc, pos| acc || (pos.0 == index && pos.1 == jndex))
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShipSet;

impl ShipSet {
    pub fn new(players: usize) -> Vec<Vec<Ship>> {
        (0..players)
            .map(|_| Ship::new_ships())
            .collect::<Vec<Vec<Ship>>>()
    }
}

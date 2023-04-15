use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FirePosition {
    pub challenge: Vec<u8>,
    pub from: usize,
    pub to: usize,
    pub lon: usize,
    pub lat: usize,
}

impl FirePosition {
    pub fn new(challenge: Vec<u8>, from: usize, to: usize, lon: usize, lat: usize) -> Self {
        Self {
            challenge,
            from,
            to,
            lon,
            lat,
        }
    }
}

const SQUARE_SIDE: usize = 10;
const PLAYERS: usize = 2;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum FiredState {
    Hit,
    Miss,
    Untouched,
    Empty,
    Ship(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Position {
    pub fired_state: Vec<FiredState>,
}

impl Position {
    pub fn new(
        lon: usize,
        lat: usize,
        shots: Option<Vec<FiredState>>,
        size: usize,
    ) -> Result<Self, String> {
        if SQUARE_SIDE - 1 < lat {
            return Err(format!(
                "Position: {}: lat {} is greater than {}",
                "lat",
                lat,
                SQUARE_SIDE - 1
            ));
        } else if SQUARE_SIDE - 1 < lon {
            return Err(format!(
                "Position: {}: lon {} is greater than {}",
                "lon",
                lon,
                SQUARE_SIDE - 1
            ));
        }
        Ok(Self {
            fired_state: shots.unwrap_or(vec![FiredState::Empty; size]),
        })
    }

    pub fn update(lat: usize, lon: usize) -> Result<Self, String> {
        if SQUARE_SIDE - 1 < lat {
            return Err(format!(
                "Position: {}: lat {} is greater than {}",
                "lat",
                lat,
                SQUARE_SIDE - 1
            ));
        } else if SQUARE_SIDE - 1 < lon {
            return Err(format!(
                "Position: {}: lon {} is greater than {}",
                "lon",
                lon,
                SQUARE_SIDE - 1
            ));
        }
        Ok(Self {
            fired_state: vec![FiredState::Untouched; PLAYERS],
        })
    }

    pub fn get_fired_state(&self) -> Vec<FiredState> {
        self.fired_state.clone()
    }
}

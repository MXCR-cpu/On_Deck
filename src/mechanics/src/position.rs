use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FirePosition {
    lat: usize,
    lon: usize,
    target: usize,
}

impl FirePosition {
    pub fn new(lat: usize, lon: usize, target: usize) -> Self {
        Self {
            lat,
            lon,
            target,
        }
    }
    pub fn print(&self) -> String {
        format!("$.board[{}][{}][{}]", self.lon, self.lat, self.target)
    }
}

const SQUARE_SIDE: usize = 10;
const PLAYERS: usize = 2;

#[derive(Serialize, Deserialize, Clone)]
pub enum FiredState {
    Hit,
    Miss,
    Untouched,
    Empty,
    Ship
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Position {
    pub fired_state: Vec<FiredState>,
}

impl Position {
    pub fn new(lat: usize, lon: usize, shots: Option<Vec<FiredState>>, size: usize) -> Result<Self, String> {
        if SQUARE_SIDE - 1 < lat {
            return Err(format!(
                "Position: {}: {} is less than zero or greater than {}",
                "lat",
                lat,
                SQUARE_SIDE - 1
            ));
        } else if SQUARE_SIDE - 1 < lon {
            return Err(format!(
                "Position: {}: {} is less than zero or greater than {}",
                "lon",
                lon,
                SQUARE_SIDE - 1
            ));
        }
        Ok(Self {
            fired_state: shots.unwrap_or(vec![FiredState::Untouched; size]),
        })
    }

    pub fn update(lat: usize, lon: usize) -> Result<Self, String> {
        if SQUARE_SIDE - 1 < lat {
            return Err(format!(
                "Position: {}: {} is less than zero or greater than {}",
                "lat",
                lat,
                SQUARE_SIDE - 1
            ));
        } else if SQUARE_SIDE - 1 < lon {
            return Err(format!(
                "Position: {}: {} is less than zero or greater than {}",
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

use rocket::serde::{Deserialize, Serialize};

// const SIZE_LIMIT: u64 = 256;
#[derive(Serialize, Deserialize)]
pub struct FirePosition {
    lat: i8,
    lon: i8,
    target: i8,
}

impl FirePosition {
    pub fn print(&self) -> String {
        format!("$.board[{}][{}][{}]", self.lon, self.lat, self.target)
    }
}

const SQUARE_SIDE: i8 = 10;
const PLAYERS: usize = 2;

#[derive(Serialize, Deserialize, Clone)]
pub struct Position {
    latitude: i8,
    longitude: i8,
    fired_state: Vec<bool>,
}

impl Position {
    pub fn new(lat: i8, lon: i8, shots: Option<Vec<bool>>) -> Result<Self, String> {
        if lat < 0 || SQUARE_SIDE - 1 < lat {
            return Err(format!(
                "Position: {}: {} is less than zero or greater than {}",
                "lat",
                lat,
                SQUARE_SIDE - 1
            ));
        } else if lon < 0 || SQUARE_SIDE - 1 < lon {
            return Err(format!(
                "Position: {}: {} is less than zero or greater than {}",
                "lon",
                lon,
                SQUARE_SIDE - 1
            ));
        }
        Ok(Self {
            latitude: lat,
            longitude: lon,
            fired_state: shots.unwrap_or(vec![false; PLAYERS]),
        })
    }

    pub fn update(lat: i8, lon: i8) -> Result<Self, String> {
        if lat < 0 || SQUARE_SIDE - 1 < lat {
            return Err(format!(
                "Position: {}: {} is less than zero or greater than {}",
                "lat",
                lat,
                SQUARE_SIDE - 1
            ));
        } else if lon < 0 || SQUARE_SIDE - 1 < lon {
            return Err(format!(
                "Position: {}: {} is less than zero or greater than {}",
                "lon",
                lon,
                SQUARE_SIDE - 1
            ));
        }
        Ok(Self {
            latitude: lat,
            longitude: lon,
            fired_state: vec![false; PLAYERS],
        })
    }

    pub fn get_fired_state(&self) -> Vec<bool> {
        self.fired_state.clone()
    }
}

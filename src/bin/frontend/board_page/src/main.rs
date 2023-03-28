use yew::classes;
use yew::prelude::*;

const SQUARE_SIDE: i8 = 10;

#[derive(Clone)]
pub struct Position {
    latitude: i8,
    longitude: i8,
    player_a_fired: bool,
    player_b_fired: bool,
}

impl Position {
    pub fn new(lat: i8, lon: i8) -> Result<Self, String> {
        if lat < 0 || SQUARE_SIDE - 1 < lat {
            return Err(format!(
                "Position: {}: {} is less than zero or greater than {}",
                "lat",
                lat,
                SQUARE_SIDE - 1
            ));
        }
        if lon < 0 || SQUARE_SIDE - 1 < lon {
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
            player_a_fired: false,
            player_b_fired: false,
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
        }
        if lon < 0 || SQUARE_SIDE - 1 < lon {
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
            player_a_fired: false,
            player_b_fired: false,
        })
    }
}

#[derive(Clone)]
struct Ship {
    name: String,
    location: Vec<Position>,
}

enum Direction {
    North,
    West,
    South,
    East,
}

impl Ship {
    pub fn new(ship_name: String, pos: Result<Vec<Position>, String>) -> Self {
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
            Self::new(
                name,
                Self::direction((0, index as i8), size, Direction::East),
            )
        })
        .collect::<Vec<Self>>() // make sure to include ship collision detection
    }

    fn direction(pos: (i8, i8), size: i8, direction: Direction) -> Result<Vec<Position>, String> {
        Ok((0..size)
            .map(|index| {
                match Position::new(
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
                ) {
                    Ok(p) => p,
                    Err(e) => panic!("{}", e),
                }
            })
            .collect::<Vec<Position>>())
    }

    fn check_hit(&self, index: i8, jndex: i8) -> bool {
        self.location.iter().fold(false, |acc, pos| {
            acc || (pos.latitude == index && pos.longitude == jndex)
        })
    }
}

struct Board {
    board: Vec<Vec<Position>>,
    player_a_ships: Vec<Ship>,
    player_b_ships: Vec<Ship>,
}

impl Board {
    pub fn initialize_board() -> Vec<Vec<Position>> {
        (0..10)
            .map(|index| {
                (0..10)
                    .map(|jndex| Position::new(index, jndex).unwrap())
                    .collect::<Vec<Position>>()
            })
            .collect::<Vec<Vec<Position>>>()
    }

    pub fn fire(&self, lat: i8, lon: i8, player: i8) -> Vec<Vec<Position>> {
        let mut new_board = self.board.clone();
        new_board[lat as usize][lon as usize] = Position {
            latitude: lat,
            longitude: lon,
            player_a_fired: player == 1 || self.board[lat as usize][lon as usize].player_a_fired,
            player_b_fired: player == 2 || self.board[lat as usize][lon as usize].player_b_fired,
        };
        new_board
    }
}

enum BoardMsg {
    PlayerAFire(i8, i8),
    PlayerBFire(i8, i8),
}

impl Component for Board {
    type Message = BoardMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            board: Board::initialize_board(),
            player_a_ships: Ship::new_ships(),
            player_b_ships: Ship::new_ships(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::PlayerAFire(lat, lon) => {
                self.board = self.fire(lat, lon, 1);
                true
            }
            Self::Message::PlayerBFire(lat, lon) => {
                self.board = self.fire(lat, lon, 2);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let indecies: Vec<(i8, i8)> = (0..10)
            .map(|index| {
                (0..10)
                    .map(|jndex| (index, jndex))
                    .collect::<Vec<(i8, i8)>>()
            })
            .collect::<Vec<Vec<(i8, i8)>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<(i8, i8)>>();
        let onclick = |index: i8, jndex: i8| {
            ctx.link()
                .callback(move |_| Self::Message::PlayerAFire(index, jndex))
        };
        let map_button_class = |result, index: i8, jndex: i8| {
            classes!(
                "main_button",
                format!("main_button_{}", result),
                format!("button_row_{}", 9 - jndex),
                format!("button_col_{}", index)
            )
        };
        html! {
            <div class={classes!("plain_style")}>
                <div class={classes!("panel")}>
                    <div class={classes!("board", "enemy_board")}>
                        {
                            indecies
                            .clone()
                            .into_iter()
                            .map(|(index, jndex)| html! {
                                if !self.board[index as usize][jndex as usize].player_a_fired {
                                    <button class={map_button_class("untouched", index, jndex)} onclick={onclick(index, jndex)}></button>
                                } else if !self.player_b_ships.iter().fold(false, |acc, ship| acc || ship.check_hit(index, jndex)){
                                   <button class={map_button_class("miss", index, jndex)}></button>
                                } else {
                                  <button class={map_button_class("hit", index, jndex)}></button>
                                }
                            })
                            .collect::<Html>()
                        }
                    </div>
                    <div class={classes!("board", "home_board")}>
                        {
                            indecies
                            .into_iter()
                            .map(|(index, jndex)| html! {
                                if self.player_a_ships.iter().fold(false, |acc, ship| acc || ship.check_hit(index, jndex)){
                                    <button class={map_button_class("ship", index, jndex)}></button>
                                } else {
                                    <button class={map_button_class("untouched", index, jndex)}></button>
                                }
                            })
                            .collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Board>::new().render();
}

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use crate::database::{
    database_get, database_set, json_database_get, json_database_set, RedisDatabase,
};
// use battleship::board::Board;
use battleship::game::Game;
use battleship::link::{GameList, GameListEntry};
use battleship::position::FirePosition;
use battleship::start;
use database::json_database_get_simple;
use rocket::{
    fairing::AdHoc,
    fs::NamedFile,
    response::{status::NotFound, Redirect},
    serde::json::Json,
};
use rocket_db_pools::{deadpool_redis::redis, Connection, Database};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

pub mod database;

#[derive(Serialize, Deserialize)]
struct NumberOfPlayers {
    number_of_players: i8,
}

const MAIN_DIR: &str = "src/bin/frontend/main_page/";
const BOARD_DIR: &str = "src/bin/frontend/board_page/";

// Utility Functions
async fn return_file(item: String) -> Result<NamedFile, NotFound<String>> {
    NamedFile::open(item)
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/<path..>")]
async fn extra_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    return_file(format!("src/bin/frontend/extra_files/{}", path.display())).await
}

// On Startup
#[get("/")]
async fn intercept_start(_rds: Connection<RedisDatabase>) -> Redirect {
    Redirect::to(uri!(main_page()))
}

//TODO: Perhaps create a unique hashing function that allows the player_id to
//be securely hidden from the client side
#[get("/get_player_id")]
async fn get_player_id(mut rds: Connection<RedisDatabase>) -> Json<u32> {
    let _res: u32 = database_get(&"player_id_count", &mut rds)
        .await
        .unwrap_or("0".to_string())
        .parse::<u32>()
        .unwrap();
    database_set(
        &vec!["player_id_count", (_res + 1).to_string().as_str()],
        &mut rds,
    )
    .await
    .unwrap();
    Json(_res)
}

#[post("/start", format = "json", data = "<players_obj>")]
async fn start_game(mut rds: Connection<RedisDatabase>, players_obj: Json<HashMap<String, u8>>) {
    //Updating EventStream call
    database_set(&vec!["links_update", "true"], &mut rds)
        .await
        .unwrap();
    //Updating Game Count Record
    let number_of_players: usize = players_obj.into_inner()["number_of_players"] as usize;
    let mut game_count: u64 = database_get(&"game_count", &mut rds)
        .await
        .unwrap_or("0".to_string())
        .parse::<u64>()
        .unwrap();
    game_count += 1;
    let new_game: Game = Game::new(number_of_players, game_count);
    database_set(
        &vec!["game_count", game_count.to_string().as_str()],
        &mut rds,
    )
    .await
    .unwrap();
    //Actually Setting Up the Game
    json_database_set::<Game>(
        &[format!("game_{}", game_count).as_str(), "."],
        &new_game,
        &mut rds,
    )
    .await
    .unwrap();
    //Updating the current game list key
    let mut current_games: GameList =
        json_database_get::<_, GameList>(&vec!["current_games", "."], &mut rds)
            .await
            .unwrap_or(Vec::new());
    current_games.push(GameListEntry::new(game_count, number_of_players));
    json_database_set::<GameList>(&["current_games", "."], &current_games, &mut rds)
        .await
        .unwrap();
}

#[get("/active_game_links")]
async fn get_active_game_links(mut rds: Connection<RedisDatabase>) -> Result<String, String> {
    match json_database_get_simple(&vec!["current_games", "."], &mut rds)
        .await {
            Ok(result) => Ok(result),
            Err(error) => {
                println!("{}", error);
                Err(error)
            }
        }
}

// Game Page Functions
//TODO: Tie the player_id to a unique player spot within game_id
//and deal with excess ids, converting them to spectators, if otherwise
#[get("/<_game_id>/<_player_id>")]
async fn process_game_request(
    _game_id: u32,
    _player_id: u32,
) -> Result<NamedFile, NotFound<String>> {
    return_file(format!("{}dist/{}", BOARD_DIR, "index.html")).await
}

#[get("/<_game_id>")]
async fn get_game_state(mut rds: Connection<RedisDatabase>, _game_id: u32) -> Result<String, String> {
    match json_database_get_simple(&vec![format!("game_{}", _game_id).as_str(), ".boards"], &mut rds)
        .await
        {
            Ok(result) => Ok(result),
            Err(error) => {
                println!("{}", error);
                Err(error)
            }
        }
}

// Board Page Functions
//TODO: Find a way to allow turn-progression tracking on the backend
//and inhibit fire request if true
#[post("/fire/<_game_id>", format = "json", data = "<fire_position>")]
async fn fire(
    mut rds: Connection<RedisDatabase>,
    fire_position: Json<FirePosition>,
    _game_id: u32,
) -> Json<String> {
    redis::cmd("JSON.SET")
        .arg(&[
            "game",
            fire_position.into_inner().print().as_str(),
            "'\"true\"'",
        ])
        .query_async::<_, ()>(&mut *rds)
        .await
        .unwrap();
    Json("Coordinate Received".to_string())
}

#[get("/<path..>")]
async fn board_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    return return_file(format!("{}dist/{}", BOARD_DIR, path.display())).await;
}

// Main Page Functions
#[get("/")]
async fn main_page() -> Result<NamedFile, NotFound<String>> {
    return_file(format!("{}dist/{}", MAIN_DIR, "index.html")).await
}

#[get("/<path..>")]
async fn main_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    return return_file(format!("{}dist/{}", MAIN_DIR, path.display())).await;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_ignite("Compiling and Configuring", |rocket| {
            Box::pin(async {
                start::build(vec![MAIN_DIR, BOARD_DIR]);
                rocket
            })
        }))
        .attach(RedisDatabase::init())
        .attach(AdHoc::on_shutdown("Stopping Docker", |_| {
            Box::pin(async {
                start::stop_rocket_database();
            })
        }))
        .mount(
            "/",
            routes![
                intercept_start,
                start_game,
                get_player_id,
                get_active_game_links
            ],
        )
        .mount("/main", routes![main_page, main_files])
        .mount("/game", routes![process_game_request, get_game_state])
        .mount("/board", routes![fire, board_files])
        .mount("/extra_files", routes![extra_files])
}

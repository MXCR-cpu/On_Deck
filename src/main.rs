#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use crate::database::{
    database_get, database_set, json_database_get, json_database_set, RedisDatabase,
};
use battleship::board::Board;
use battleship::game::Game;
use battleship::position::FirePosition;
use battleship::start;

use rocket::{
    fs::NamedFile,
    response::{status::NotFound, Redirect},
    serde::json::Json,
};
use rocket_db_pools::{deadpool_redis::redis, Connection, Database};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

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

//To-Do: Perhaps create a unique hashing function that allows the player_id to
//be securely hidden from the client side
#[get("/get_player_id")]
async fn get_player_id(mut rds: Connection<RedisDatabase>) -> Json<u32> {
    let _res: u32 = redis::cmd("GET")
        .arg(&["player_id_count"])
        .query_async(&mut *rds)
        .await
        .unwrap_or(0);
    redis::cmd("SET")
        .arg(&["player_id_count", (_res + 1).to_string().as_str()])
        .query_async::<_, ()>(&mut *rds)
        .await
        .unwrap();
    Json(_res)
}

#[post("/start", format = "json", data = "<players_obj>")]
async fn start_game(mut rds: Connection<RedisDatabase>, players_obj: Json<HashMap<String, u8>>) {
    let players_number: u8 = players_obj.into_inner()["number_of_players"];
    let mut game_count: u64 = database_get::<&str>("game_count", &mut rds)
        .await
        .unwrap_or("0".to_string())
        .parse::<u64>()
        .unwrap();
    game_count += 1;
    let new_game: Game = Game::new(players_number, game_count);
    database_set::<&[&str]>(&["game_count", game_count.to_string().as_str()], &mut rds)
        .await
        .unwrap();
    json_database_set::<Game>(
        &[format!("game_{}", game_count).as_str(), "."],
        &new_game,
        &mut rds,
    )
    .await
    .unwrap();
    let mut current_games: Vec<String> =
        json_database_get::<&[&str], Vec<String>>(&["current_games", "."], &mut rds)
            .await
            .unwrap_or(Vec::new());
    current_games.push(game_count.to_string());
    json_database_set::<Vec<String>>(&["current_games", "."], &current_games, &mut rds)
        .await
        .unwrap();
}

#[get("/game_links")]
async fn get_game_links(mut rds: Connection<RedisDatabase>) -> Json<Vec<String>> {
    let active_games: Vec<String> =
        json_database_get::<&[&str], Vec<String>>(&["current_games", "."], &mut rds)
            .await
            .unwrap_or(Vec::new());
    println!("{:?}", active_games);
    Json(active_games)
}

// Game Page Functions
//To-Do: Tie the player_id to a unique player spot within game_id
//and deal with excess ids, converting them to spectators, if otherwise
#[get("/<_game_id>/<_player_id>")]
async fn process_game_request(
    _game_id: u32,
    _player_id: u32,
) -> Result<NamedFile, NotFound<String>> {
    return_file(format!("{}dist/{}", BOARD_DIR, "index.html")).await
}

#[get("/<_game_id>")]
async fn get_game_state(
    mut rds: Connection<RedisDatabase>,
    _game_id: u32,
) -> Option<Json<Vec<Board>>> {
    match json_database_get::<&[&str], Vec<Board>>(
        &[format!("game_{}", _game_id).as_str(), "$.boards"],
        &mut rds,
    )
    .await
    {
        Some(vec_res) => Some(Json(vec_res)),
        None => None,
    }
}

// Board Page Functions
//To-Do: Find a way to allow turn-progression tracking on the backend
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
    start::build(vec![MAIN_DIR, BOARD_DIR]);
    rocket::build()
        .attach(RedisDatabase::init())
        .mount(
            "/",
            routes![intercept_start, start_game, get_player_id, get_game_links,],
        )
        .mount("/main", routes![main_page, main_files])
        .mount("/game", routes![process_game_request, get_game_state])
        .mount("/board", routes![fire, board_files])
        .mount("/extra_files", routes![extra_files])
}

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use crate::database::RedisDatabase;
use battleship::game::Game;
use battleship::board::Board;
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

//To-Do: Add a way to increment a game-count counter and save that to a list of saved games
//all the while utilizing it as keyword for indexing the game
#[post("/start", format = "json", data = "<players_obj>")]
async fn start_game(mut rds: Connection<RedisDatabase>, players_obj: Json<HashMap<String, u8>>) {
    // This needs to be simplified
    let players_number: u8 = players_obj.into_inner()["number_of_players"];
    // println!("{{\"number_of_players\": {}}}", players_number);
    let new_game: Game = Game::new(players_number);
    redis::cmd("JSON.SET")
        .arg(&[
            "game",
            ".",
            serde_json::to_string(&new_game)
                .unwrap()
                .as_str(),
        ])
        .query_async::<_, ()>(&mut *rds)
        .await
        .unwrap();
}

//To-Do: Find a away to iterate over all possible games' links and ship them to
//the client so that way they can be displayed
#[get("/game_links")]
async fn get_game_links(mut rds: Connection<RedisDatabase>) -> Json<Vec<String>> {
    let redis_query: String = redis::cmd("JSON.GET")
        .arg(&["game", "$.game_link"])
        .query_async(&mut *rds)
        .await
        .unwrap();
    let active_links_new: String =
        serde_json::from_str::<Vec<String>>(redis_query.as_str()).unwrap()[0].clone();
    println!("{}", active_links_new);
    Json(vec![active_links_new])
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
async fn get_game_state(mut rds: Connection<RedisDatabase>, _game_id: u32) -> Json<Vec<Board>> {
    let _res: String = redis::cmd("JSON.GET")
        .arg(&["game", "$.boards"])
        .query_async(&mut *rds)
        .await
        .unwrap();
    let _vec_res: Vec<Board> = serde_json::from_str(_res.as_str()).unwrap();
    Json(_vec_res)
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
            routes![
                intercept_start,
                start_game,
                get_player_id,
                get_game_links,
            ],
        )
        .mount("/main", routes![main_page, main_files])
        .mount("/game", routes![process_game_request, get_game_state])
        .mount("/board", routes![fire, board_files])
        .mount("/extra_files", routes![extra_files])
}

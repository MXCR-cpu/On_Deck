#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use crate::board::Board;
use crate::position::FirePosition;
use rand::prelude::*;
use rocket::{
    fs::NamedFile,
    response::{status::NotFound, Redirect},
    serde::json::Json,
};
use rocket_db_pools::{
    deadpool_redis::{redis, redis::AsyncCommands, Pool},
    Connection, Database,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub mod board;
pub mod position;
pub mod ship;
mod start;

#[derive(Database)]
#[database("redis")]
struct RedisDatabase(Pool);

#[derive(Serialize, Deserialize)]
struct Links {
    hyperlinks: Vec<String>,
}

impl Links {
    pub fn new(number_of_players: i8) -> Self {
        Self {
            hyperlinks: Self::create_links(number_of_players),
        }
    }

    fn create_links(number_of_players: i8) -> Vec<String> {
        let mut stack: Vec<i32> = Vec::new();
        let mut rng = rand::thread_rng();
        (0..number_of_players)
            .map(|_| {
                let number: i32 = (rng.gen::<f64>() * 1000.0) as i32;
                stack.push(number);
                format!("http://127.0.0.1:8000/board/{}", number)
            })
            .collect::<Vec<String>>()
    }
}

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

// This needs to be fixed
#[get("/board_state")]
async fn get_board_state(mut rds: Connection<RedisDatabase>) -> () {
    let _res = redis::cmd("JSON.GET")
        .arg(&["game", "$.board"])
        .query_async::<rocket_db_pools::deadpool_redis::Connection, ()>(&mut *rds)
        .await
        .unwrap();
    ()
}

#[post("/start", format = "json", data = "<players_obj>")]
async fn start_game(
    mut rds: Connection<RedisDatabase>,
    players_obj: Json<HashMap<String, i8>>,
) {
    let players_number: i8 = players_obj.into_inner()["number_of_players"];
    redis::cmd("JSON.SET")
        .arg(&[
            "game",
            ".",
            serde_json::to_string(&Board::new(players_number.clone()))
                .unwrap()
                .as_str(),
        ])
        .query_async::<_, ()>(&mut *rds)
        .await
        .unwrap();
    let stored_links: Links = Links::new(players_number.clone());
    let json_stored_links: String = serde_json::to_string(&stored_links).unwrap();
    redis::cmd("SET")
        .arg(&[
            "connections",
            json_stored_links.as_str(),
        ])
        .query_async::<_, ()>(&mut *rds)
        .await
        .unwrap();
}

#[get("/links")]
async fn get_links(mut rds: Connection<RedisDatabase>) -> Json<String> {
    let active_links: String = rds.get("connections").await.unwrap();
    Json(active_links)
}

// Board Page Functions
#[get("/<_id>")]
async fn board_page(_id: u32) -> Result<NamedFile, NotFound<String>> {
    return_file(format!("{}dist/{}", BOARD_DIR, "index.html")).await
}

#[post("/fire/<_id>", format = "json", data = "<fire_position>")]
async fn fire(mut rds: Connection<RedisDatabase>, fire_position: Json<FirePosition>, _id: u32) -> Json<String> {
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

#[get("/<path..>", rank = 2)]
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
            routes![intercept_start, start_game, get_links, get_board_state],
        )
        .mount("/main", routes![main_page, main_files])
        .mount("/board", routes![board_page, fire, board_files])
        .mount("/extra_files", routes![extra_files])
}

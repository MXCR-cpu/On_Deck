#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use crate::database::{
    database_get, database_set, json_database_get, json_database_set, RedisDatabase,
};
use battleship::keys::PlayerKeys;
use battleship::start;
use database::json_database_get_simple;
use ecies::decrypt;
use interact::link::{GameList, GameListEntry};
use interact::site::SITE_LINK;
use mechanics::game::Game;
use mechanics::position::FirePosition;
use rand::{distributions::Alphanumeric, Rng};
use rocket::Shutdown;
use rocket::{
    fairing::AdHoc,
    fs::NamedFile,
    response::{
        status::NotFound,
        stream::{Event, EventStream},
        Redirect,
    },
    serde::json::Json,
    tokio::select,
};
use rocket_db_pools::{Connection, Database};
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
async fn intercept_start() -> Redirect {
    Redirect::to(format!("{SITE_LINK}/main"))
}

//TODO: Perhaps create a unique hashing function that allows the player_id to
//be securely hidden from the client side
#[get("/get_player_id")]
async fn get_player_id(mut rds: Connection<RedisDatabase>) -> Json<(String, String)> {
    let res: u32 = database_get(&"player_id_count", &mut rds)
        .await
        .unwrap_or("0".to_string())
        .parse::<u32>()
        .unwrap();
    let player_index: String = format!("player_{res}");
    database_set(
        &vec!["player_id_count", (res + 1).to_string().as_str()],
        &mut rds,
    )
    .await
    .unwrap();
    let player_keys: PlayerKeys = PlayerKeys::new();
    player_keys.log(&player_index);
    json_database_set(&vec![player_index.as_str(), "."], &player_keys, &mut rds)
        .await
        .unwrap();
    Json((player_index, player_keys.public_key_string()))
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
    database_set(&vec!["main_page_update", "true"], &mut rds)
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

#[get("/page_stream")]
async fn get_page_stream(
    mut rds: Connection<RedisDatabase>,
    mut shutdown: Shutdown,
) -> EventStream![] {
    EventStream! {
        loop {
            select! {
                _ = &mut shutdown => {
                    yield Event::data("end");
                    break;
                }
                value = database_get(&"links_update", &mut rds) => {
                    if value.unwrap_or("false".to_string()).eq("true") {
                        database_set(&vec!["links_update", "false"], &mut rds).await.unwrap();
                        yield Event::data("");
                    }
                }
            }
        }
    }
}

#[get("/active_game_links")]
async fn get_active_game_links(mut rds: Connection<RedisDatabase>) -> Result<String, String> {
    match json_database_get_simple(&vec!["current_games", "."], &mut rds).await {
        Ok(result) => Ok(result),
        Err(error) => {
            println!("{}", error);
            Err(error)
        }
    }
}

// Game Page Functions
#[get("/<game_id>/<player_id>")]
async fn process_game_request(
    mut rds: Connection<RedisDatabase>,
    game_id: u32,
    player_id: String,
) -> Result<NamedFile, NotFound<String>> {
    let game_tag: String = format!("game_{game_id}");
    let mut game_state: Game =
        match json_database_get(&vec![format!("game_{game_id}").as_str(), "."], &mut rds).await {
            Ok(boards) => boards,
            Err(error) => {
                println!("{}", error);
                panic!()
            }
        };
    // Passing game information if it is already filled
    if game_state.player_tags.len() == game_state.number_of_players
        || game_state.player_tags.contains(&player_id)
    {
        return return_file(format!("{BOARD_DIR}dist/index.html")).await;
    }
    game_state.player_tags.push(player_id);
    json_database_set::<Vec<String>>(
        &[
            "current_games",
            &format!(".[{}].active_player_names", game_id - 1),
        ],
        &game_state.player_tags,
        &mut rds,
    )
    .await
    .unwrap();
    if game_state.player_tags.len() < game_state.number_of_players {
        json_database_set::<Game>(&vec![game_tag.as_str(), "."], &game_state, &mut rds)
            .await
            .unwrap();
        return return_file(format!("{BOARD_DIR}dist/index.html")).await;
    }
    // Kick-off the game by creating firing create challenge
    game_state.challenge = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect::<String>();
    game_state.boards.start_board();
    database_set(
        &vec![format!("game_update_{game_id}").as_str(), "true"],
        &mut rds,
    )
    .await
    .unwrap();
    json_database_set::<Game>(&vec![game_tag.as_str(), "."], &game_state, &mut rds)
        .await
        .unwrap();
    return_file(format!("{BOARD_DIR}dist/index.html")).await
}

#[get("/<game_id>/<player_id>/<access_key>")]
async fn get_game_state(
    mut rds: Connection<RedisDatabase>,
    game_id: u32,
    player_id: String,
    access_key: String,
) -> Json<(String, String, String, String)> {
    let game_state: Game =
        match json_database_get(&vec![format!("game_{game_id}").as_str(), "."], &mut rds).await {
            Ok(boards) => boards,
            Err(error) => {
                println!("{}", error);
                panic!()
            }
        };
    let player_tags_string: String = serde_json::to_string(&game_state.player_tags).unwrap();
    // Spector mode if already filled
    if !game_state.player_tags.contains(&player_id)
        && game_state.player_tags.len() == game_state.number_of_players
    {
        return Json((
            "".to_string(),
            player_tags_string,
            "".to_string(),
            serde_json::to_string(&game_state.boards.positions).unwrap(),
        ));
    }
    let decrypt_key: Vec<u8> =
        json_database_get(&vec![player_id.as_str(), ".decryption_key"], &mut rds)
            .await
            .unwrap();
    let vec_access_key: Vec<u8> = (0..access_key.len() / 2)
        .map(|index: usize| {
            i64::from_str_radix(&access_key[(2 * index)..(2 * index) + 2], 16).unwrap() as u8
        })
        .collect::<Vec<u8>>();
    let key_result: String = std::str::from_utf8(&decrypt(&decrypt_key, &vec_access_key).unwrap())
        .unwrap()
        .to_string();
    if !key_result.eq("Request") || game_state.challenge.eq("") {
        println!("Key Failed to Triggered for {player_id}: Key {key_result}");
        return Json((
            game_state.challenge,
            player_tags_string,
            "".to_string(),
            serde_json::to_string(&game_state.boards.positions).unwrap(),
        ));
    }
    let player_index: usize = game_state
        .player_tags
        .iter()
        .position(|x| player_id.eq(x))
        .unwrap();
    Json((
        game_state.challenge,
        player_tags_string,
        serde_json::to_string(&game_state.boards.ship_set[player_index]).unwrap(),
        serde_json::to_string(
            &game_state
                .boards
                .get_board_with_player_positions(player_index),
        )
        .unwrap(),
    ))
}

#[get("/<game_number>/game_stream")]
async fn get_game_stream(
    mut rds: Connection<RedisDatabase>,
    mut shutdown: Shutdown,
    game_number: usize,
) -> EventStream![] {
    let game_tag: String = format!("game_update_{game_number}");
    EventStream! {
        loop {
            select! {
                _ = &mut shutdown => {
                    yield Event::data("end");
                    break;
                }
                value = database_get(&game_tag, &mut rds) => {
                    if value.unwrap_or("false".to_string()).eq("true") {
                        database_set(&vec![game_tag.as_str(), "false"], &mut rds).await.unwrap();
                        yield Event::data("");
                    }
                }
            }
        }
    }
}

//TODO: Find a way to allow turn-progression tracking on the backend
//and inhibit fire request if true
#[post("/fire/<game_id>", format = "json", data = "<fire_position_json>")]
async fn fire(
    mut rds: Connection<RedisDatabase>,
    fire_position_json: Json<FirePosition>,
    game_id: u32,
) -> Json<bool> {
    let fire_position: FirePosition = fire_position_json.into_inner();
    let game_tag: String = format!("game_{game_id}");
    let mut game_state: Game =
        match json_database_get(&vec![game_tag.as_str(), "."], &mut rds).await {
            Ok(boards) => boards,
            Err(error) => {
                println!("{}", error);
                panic!()
            }
        };
    if (game_state.shot_list & (1 << fire_position.from)) != 0 {
        return Json(false);
    }
    let decrypt_key: Vec<u8> = json_database_get(
        &vec![
            game_state.player_tags[fire_position.from].as_str(),
            ".decryption_key",
        ],
        &mut rds,
    )
    .await
    .unwrap();
    if !game_state.challenge.eq(&std::str::from_utf8(
        &decrypt(&decrypt_key, &fire_position.challenge).unwrap(),
    )
    .unwrap())
    {
        return Json(false);
    }
    game_state.shot_list = game_state.shot_list & (1 << fire_position.from);
    format!("{:b}", game_state.shot_list);
    game_state.boards.positions = game_state
        .boards
        .fire(fire_position.lon, fire_position.lat, fire_position.to)
        .unwrap();
    if game_state.shot_list
        ^ (2_u32
            .checked_pow(game_state.number_of_players as u32)
            .unwrap()
            - 1)
        == 0
    {
        database_set(
            &vec![format!("game_update_{game_id}").as_str(), "true"],
            &mut rds,
        )
        .await
        .unwrap();
        game_state.shot_list = 0;
    }
    json_database_set::<Game>(&vec![game_tag.as_str(), "."], &game_state, &mut rds)
        .await
        .unwrap();
    Json(true)
}

#[get("/<path..>")]
async fn board_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    return return_file(format!("{BOARD_DIR}dist/{}", path.display())).await;
}

// Main Page Functions
#[get("/")]
async fn main_page() -> Result<NamedFile, NotFound<String>> {
    return_file(format!("{MAIN_DIR}dist/index.html")).await
}

#[get("/<path..>")]
async fn main_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    return return_file(format!("{MAIN_DIR}dist/{}", path.display())).await;
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
        .mount("/main", routes![get_page_stream, main_page, main_files])
        .mount(
            "/game",
            routes![get_game_stream, fire, process_game_request, get_game_state],
        )
        .mount("/board", routes![fire, board_files])
        .mount("/extra_files", routes![extra_files])
}

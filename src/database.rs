use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use rocket_db_pools::{
    deadpool_redis::{
        Pool,
        redis,
        redis::ToRedisArgs,
        redis::RedisError,
    },
    Connection,
    Database,
};

#[derive(Database)]
#[database("redis")]
pub struct RedisDatabase(Pool);

pub async fn database_get<B: ToRedisArgs>(arg: B, rds: &mut Connection<RedisDatabase>) -> Result<String, RedisError> {
    redis::cmd("GET")
        .arg(arg)
        .query_async::<_, String>(&mut **rds)
        .await
}

pub async fn database_set<B: ToRedisArgs>(arg: B, rds: &mut Connection<RedisDatabase>) -> Result<(), RedisError> {
    redis::cmd("SET")
        .arg(arg)
        .query_async::<_, ()>(&mut **rds)
        .await
        .unwrap();
    Ok(())
}

pub async fn json_database_get<B: ToRedisArgs, T: DeserializeOwned>(arg: B, rds: &mut Connection<RedisDatabase>) -> Option<T> {
    match redis::cmd("JSON.GET")
        .arg(arg)
        .query_async::<_, String>(&mut **rds)
        .await {
            Ok(value) => Some(serde_json::from_str::<T>(value.as_str()).unwrap()),
            Err(_) => None,
    }
}

pub async fn json_database_set<T: Serialize>(arg: &[&str], item: &T, rds: &mut Connection<RedisDatabase>) -> Result<(), RedisError> {
    let mut str_vec: Vec<&str> = Vec::new();
    for element in arg.into_iter() {
        str_vec.push(element);
    }
    let json_item: String = serde_json::to_string(&item).unwrap();
    str_vec.push(json_item.as_str());
    redis::cmd("JSON.SET")
        .arg(str_vec)
        .query_async::<_, ()>(&mut **rds)
        .await
        .unwrap();
    Ok(())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameListEntry {
    game_record_number: u64,
    total_positions: u8,
    active_player_names: Vec<String>,
}

impl GameListEntry {
    pub fn new(game_record_number: u64, total_positions: u8) -> Self {
        Self {
            game_record_number,
            total_positions,
            active_player_names: Vec::new(),
        }
    }

    pub fn add_player(&self, player_name: String) -> Result<Self, &str> {
        if self.is_full() {
            return Err("Game is already full");
        }
        let mut player_list: Vec<String> = self.active_player_names.clone();
        player_list.push(player_name);
        Ok(Self {
            game_record_number: self.game_record_number,
            total_positions: self.total_positions,
            active_player_names: player_list,
        })
    }

    fn is_full(&self) -> bool {
        self.active_player_names.len() as u8 == self.total_positions
    }
}

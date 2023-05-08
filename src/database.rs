use rocket_db_pools::{
    deadpool_redis::{redis, redis::ToRedisArgs, Pool},
    Connection, Database,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Database)]
#[database("redis")]
pub struct RedisDatabase(Pool);

pub enum DatabaseOption {
    GET,
    SET,
    RENAME,
}

impl std::fmt::Display for DatabaseOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::GET => "GET",
            Self::SET => "SET",
            Self::RENAME => "RENAME",
        })
    }
}

pub async fn database<T: ToRedisArgs>(
    option: DatabaseOption,
    args: &T,
    rds: &mut Connection<RedisDatabase>) -> Result<String, String> {
    match redis::cmd(&format!("{}", option))
        .arg(&args)
        .query_async::<_, String>(&mut **rds)
        .await
    {
        Ok(result) => Ok(result),
        Err(error) => Err(format!(
                "database.rs: database({}): Redis Cmd Failed to execute option query command; {}",
                option,
                error
            )),
    }
}

pub async fn database_get<T: ToRedisArgs>(
    args: &T,
    rds: &mut Connection<RedisDatabase>,
) -> Result<String, String> {
    match redis::cmd("GET")
        .arg(&args)
        .query_async::<_, String>(&mut **rds)
        .await
    {
        Ok(result) => Ok(result),
        Err(error) => Err(format!(
            "database.rs: database_get(): Redis Cmd Failed to execute `GET` query command; {}",
            error
        )),
    }
}

pub async fn database_set<T: ToRedisArgs>(
    args: &T,
    rds: &mut Connection<RedisDatabase>,
) -> Result<(), String> {
    match redis::cmd("SET")
        .arg(args)
        .query_async::<_, ()>(&mut **rds)
        .await
    {
        Ok(_) => Ok(()),
        Err(error) => Err(format!(
            "database.rs: database_set(): Redis Cmd Failed to execute `SET` query command; {}",
            error
        )),
    }
}

pub async fn database_rename(
    args: &Vec<String>,
    rds: &mut Connection<RedisDatabase>,
) -> Result<(), String> {
    match redis::cmd("RENAME")
        .arg(args)
        .query_async::<_, String>(&mut **rds)
        .await
        {
            Ok(_) => Ok(()),
            Err(error) => Err(format!(
                "database.rs: database_rename(): Redis Cmd Failed to execute `RENAME` query command; {}",
                error
            ))
        }
}

pub async fn json_database_get_simple<T: ToRedisArgs>(
    args: &T,
    rds: &mut Connection<RedisDatabase>,
) -> Result<String, String> {
    match redis::cmd("JSON.GET")
        .arg(&args)
        .query_async::<_, String>(&mut **rds)
        .await
    {
        Ok(result) => Ok(result),
        Err(error) => Err(format!(
            "database.rs, 47: Redis Cmd Failed to execute `JSON.GET` query command; {}",
            error
        )),
    }
}

pub async fn json_database_get<T: ToRedisArgs, D: DeserializeOwned>(
    args: &T,
    rds: &mut Connection<RedisDatabase>,
) -> Result<D, String> {
    match redis::cmd("JSON.GET")
        .arg(&args)
        .query_async::<_, String>(&mut **rds)
        .await
    {
        Ok(result) => match serde_json::from_str::<D>(result.as_str()) {
            Ok(result) => Ok(result),
            Err(error) => Err(format!(
                "database.rs, 43: serde_json Failed to deserialize `result.as_str()`; {}",
                error
            )),
        },
        Err(error) => Err(format!(
            "database.rs, 38: Redis Cmd Failed to execute `JSON.GET` query command; {}",
            error
        )),
    }
}

pub async fn json_database_set<D: Serialize + std::marker::Sync>(
    args: &[&str],
    item: &D,
    rds: &mut Connection<RedisDatabase>,
) -> Result<(), String> {
    let mut str_vec: Vec<&str> = args.to_vec();
    let json_item: String = match serde_json::to_string(&item) {
        Ok(result) => result,
        Err(error) => {
            return Err(format!(
                "database.rs, 63: Failed to serialize `item` object; {}",
                error
            ))
        }
    };
    str_vec.push(json_item.as_str());
    match redis::cmd("JSON.SET")
        .arg(str_vec)
        .query_async::<_, ()>(&mut **rds)
        .await
    {
        Ok(_) => Ok(()),
        Err(error) => Err(format!(
            "database.rs: 73: Redis Cmd Failed to execute `JSON.SET` query command; {}",
            error
        )),
    }
}

use rocket_db_pools::{
    deadpool_redis::{redis, redis::ToRedisArgs, Pool},
    Connection, Database,
};
// use serde::de::DeserializeOwned;
// use serde::Serialize;

#[derive(Database)]
#[database("redis")]
pub struct RedisDatabase(Pool);

pub fn evocation() -> String {
    format!("{}, {}", file!().to_string(), line!().to_string())
}

pub enum DatabaseOption {
    GET,
    SET,
    RENAME,
}

impl std::fmt::Display for DatabaseOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::GET => "GET",
                Self::SET => "SET",
                Self::RENAME => "RENAME",
            }
        )
    }
}

pub async fn database<T: ToRedisArgs>(
    option: DatabaseOption,
    args: &T,
    rds: &mut Connection<RedisDatabase>,
) -> Result<String, String> {
    match redis::cmd(&format!("{}", option))
        .arg(&args)
        .query_async::<_, String>(&mut **rds)
        .await
    {
        Ok(result) => Ok(result),
        Err(error) => Err(format!(
            "database.rs: database({}): Redis Cmd Failed to execute option query command; {}",
            option, error
        )),
    }
}

pub async fn json_database(
    option: DatabaseOption,
    args: &Vec<String>,
    rds: &mut Connection<RedisDatabase>,
) -> Result<String, String> {
    match redis::cmd(&format!("JSON.{}", option))
        .arg(&args)
        .query_async::<_, String>(&mut **rds)
        .await
    {
        Ok(result) => Ok(result),
        Err(error) => Err(format!(
            "database.rs, 47: Redis Cmd Failed to execute `JSON.{}` query command; {}",
            option, error,
        )),
    }
}

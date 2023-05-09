use rocket_db_pools::{
    deadpool_redis::{redis, redis::ToRedisArgs, Pool},
    Connection, Database,
};

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
            "{}, {}: database({}): Redis Cmd Failed to execute option query command; {}",
            file!(),
            line!(),
            option,
            error
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
            "{}, {}: json_database({}): Redis Cmd Failed to execute `JSON.{}` query command; {}",
            file!(),
            line!(),
            option,
            option,
            error,
        )),
    }
}

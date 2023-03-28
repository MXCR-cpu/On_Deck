use std::time::Duration;
use rocket::{Rocket, Build};
use rocket_sync_db_pools::{r2d2, r2d2::ManageConnection, Error, Config, Poolable, PoolResult};
use r2d2_redis::{redis, RedisConnectionManager};

pub struct RedisConnection;

#[cfg(feature = "redis_pool")]
impl Poolable for RedisConnection {
    type Manager = RedisConnectionManager;
    type Error = redis::ErrorKind;

    fn pool(db_name: &str, rocket: &Rocket<Build>) -> PoolResult<Self> {
    let config = Config::from(db_name, rocket)?;
    let manager = RedisConnectionManager::new(&*config.url).map_err(Error::Custom)?;
    Ok(r2d2::Pool::builder()
       .max_size(config.pool_size)
       .connection_timeout(Duration::from_secs(config.timeout as u64))
       .build(manager)?)
    }
}

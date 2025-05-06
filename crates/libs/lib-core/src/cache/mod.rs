use super::error::Result;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;

pub mod redis_fns;

pub type Cache = Pool<RedisConnectionManager>;

pub async fn new_cache_pool() -> Result<Cache> {
    let manager = RedisConnectionManager::new("redis://127.0.0.1:6379").unwrap();
    let pool = Pool::builder().build(manager).await?;

    Ok(pool)
}

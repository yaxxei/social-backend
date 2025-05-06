use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::Result;

use super::Cache;

pub async fn set<T>(redis_pool: &Cache, key: &str, value: T, ttl_sec: Option<usize>) -> Result<()>
where
    T: Serialize + Send + Sync,
{
    let mut conn = redis_pool.get().await.unwrap();

    let value_str = serde_json::to_string(&value)?;

    if let Some(ttl_seconds) = ttl_sec {
        let _: () = conn
            .set_ex(key.to_owned(), value_str, ttl_seconds as u64)
            .await?;
    } else {
        let _: () = conn.set(key.to_owned(), value_str).await?;
    }

    Ok(())
}

pub async fn get<T>(redis_pool: &Cache, key: &str) -> Result<Option<T>>
where
    T: DeserializeOwned + Send + Sync,
{
    let mut conn = redis_pool.get().await.unwrap();

    let result: String = conn.get(key).await?;

    let value = serde_json::from_str(&result)?;

    Ok(value)
}

#[cfg(test)]
mod test {
    use crate::cache::{
        new_cache_pool,
        redis_fns::{get, set},
    };
    use anyhow::Result;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestValue {
        data: String,
    }

    #[tokio::test]
    async fn test_set_cache_with_struct() -> Result<()> {
        let pool = new_cache_pool().await?;

        let key = "test_key";
        let value = TestValue {
            data: "Hello, Redis!".to_string(),
        };

        set(&pool, key, value.clone(), Some(10)).await?;

        let value = get::<TestValue>(&pool, key).await?;
        assert_eq!(value.unwrap().data, "Hello, Redis!".to_string());

        Ok(())
    }

    #[tokio::test]
    async fn test_set_cache_with_str() -> Result<()> {
        let pool = new_cache_pool().await?;

        let key = "test_key";
        let data = "Hello, Redis!";

        set(&pool, key, data.to_string(), Some(10)).await?;

        let value = get::<String>(&pool, key).await?;
        assert_eq!(value.unwrap(), data.to_string());

        Ok(())
    }
}

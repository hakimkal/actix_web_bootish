use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::sync::Mutex;
use once_cell::sync::OnceCell;
use crate::config::AppConfig;

pub struct RedisService {
    connection: Mutex<MultiplexedConnection>,
}

static INSTANCE: OnceCell<RedisService> = OnceCell::new();

impl RedisService {
    /// Initialize RedisService only once
    pub async fn init() {
        if INSTANCE.get().is_none() {
            let redis_url = AppConfig::from_env().redis_cache_url.clone();
            let client = Client::open(redis_url).expect("Failed to create Redis client");

            let connection = client.get_multiplexed_async_connection().await.expect("Failed to create Redis connection");

            INSTANCE.set(RedisService {
                connection: Mutex::new(connection),
            }).ok();
        }
    }

    /// Get the RedisService instance
    pub fn instance() -> &'static RedisService {
        INSTANCE.get().expect("RedisService is not initialized. Call RedisService::init() first")
    }

    /// Set value in Redis (Generic)
    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl: u64) -> redis::RedisResult<()> {
        let mut con = self.connection.lock().await;
        let json_value = serde_json::to_string(value).map_err(|_| redis::RedisError::from((
            redis::ErrorKind::TypeError,
            "Failed to serialize object to JSON",
        )))?;

        con.set_ex(key, json_value, ttl).await
    }

    /// Get value from Redis (Generic)
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> redis::RedisResult<Option<T>> {
        let mut con = self.connection.lock().await;
        let json_data: Option<String> = con.get(key).await?;
        match json_data {
            Some(data) => {
                let parsed = serde_json::from_str(&data).map_err(|_| redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "Failed to deserialize JSON",
                )))?;
                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }

    pub async fn delete(&self, key: &str) -> redis::RedisResult<u64> {
        let mut con = self.connection.lock().await;
        let deleted_count: u64 = con.del(key).await?;
        Ok(deleted_count)
    }
}

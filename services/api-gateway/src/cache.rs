use anyhow::Result;
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info};

#[derive(Clone)]
pub struct CacheService {
    client: Client,
}

impl CacheService {
    pub async fn new(redis_url: &str) -> Result<Self> {
        info!("Connecting to Redis cache...");

        let client = Client::open(redis_url)?;

        // Test connection
        let mut conn = client.get_async_connection().await?;
        let _: String = redis::cmd("PING").query_async(&mut conn).await?;

        info!("Redis cache connection established");
        Ok(Self { client })
    }

    pub async fn health_check(&self) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        let _: String = redis::cmd("PING").query_async(&mut conn).await?;
        Ok(())
    }

    pub async fn get<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut conn = self.client.get_async_connection().await?;
        let value: Option<String> = conn.get(key).await?;

        match value {
            Some(json_str) => {
                let deserialized: T = serde_json::from_str(&json_str)?;
                debug!("Cache hit for key: {}", key);
                Ok(Some(deserialized))
            }
            None => {
                debug!("Cache miss for key: {}", key);
                Ok(None)
            }
        }
    }

    pub async fn set<T>(&self, key: &str, value: &T, ttl: Option<Duration>) -> Result<()>
    where
        T: Serialize,
    {
        let mut conn = self.client.get_async_connection().await?;
        let json_str = serde_json::to_string(value)?;

        match ttl {
            Some(duration) => {
                let seconds = duration.as_secs().try_into().unwrap_or(usize::MAX);
                let _: () = conn.set_ex(key, json_str, seconds).await?;
            }
            None => {
                let _: () = conn.set(key, json_str).await?;
            }
        }

        debug!("Cached value for key: {}", key);
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        let _: i32 = conn.del(key).await?;
        debug!("Deleted cache key: {}", key);
        Ok(())
    }

    pub async fn exists(&self, key: &str) -> Result<bool> {
        let mut conn = self.client.get_async_connection().await?;
        let exists: bool = conn.exists(key).await?;
        Ok(exists)
    }

    // Session management
    pub async fn store_session(&self, session_id: &str, user_id: &str, ttl: Duration) -> Result<()> {
        self.set(&format!("session:{}", session_id), &user_id, Some(ttl)).await
    }

    pub async fn get_session(&self, session_id: &str) -> Result<Option<String>> {
        self.get(&format!("session:{}", session_id)).await
    }

    pub async fn invalidate_session(&self, session_id: &str) -> Result<()> {
        self.delete(&format!("session:{}", session_id)).await
    }

    // Rate limiting
    pub async fn increment_rate_limit(&self, key: &str, window: Duration) -> Result<i32> {
        let mut conn = self.client.get_async_connection().await?;
        let count: i32 = conn.incr(key, 1).await?;

        if count == 1 {
            let seconds = window.as_secs().try_into().unwrap_or(usize::MAX);
            let _: () = conn.expire(key, seconds).await?;
        }

        Ok(count)
    }
}

// Cache module placeholder

use redis::{Client, AsyncCommands};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub struct CacheService {
    client: Client,
}

impl CacheService {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }

    pub async fn get<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut conn = self.client.get_async_connection().await?;
        let value: Option<String> = conn.get(key).await?;
        
        match value {
            Some(v) => Ok(Some(serde_json::from_str(&v)?)),
            None => Ok(None),
        }
    }

    pub async fn set<T>(&self, key: &str, value: &T, ttl: Option<usize>) -> Result<()>
    where
        T: Serialize,
    {
        let mut conn = self.client.get_async_connection().await?;
        let serialized = serde_json::to_string(value)?;
        
        if let Some(ttl) = ttl {
            conn.set_ex(key, serialized, ttl).await?;
        } else {
            conn.set(key, serialized).await?;
        }
        
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        conn.del(key).await?;
        Ok(())
    }

    pub async fn exists(&self, key: &str) -> Result<bool> {
        let mut conn = self.client.get_async_connection().await?;
        let exists: bool = conn.exists(key).await?;
        Ok(exists)
    }
}

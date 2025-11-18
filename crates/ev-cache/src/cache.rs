use redis::{aio::ConnectionManager, AsyncCommands, RedisError};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Redis connection error: {0}")]
    Connection(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Cache operation error: {0}")]
    Operation(String),
}

impl From<RedisError> for CacheError {
    fn from(err: RedisError) -> Self {
        CacheError::Operation(err.to_string())
    }
}

impl From<serde_json::Error> for CacheError {
    fn from(err: serde_json::Error) -> Self {
        CacheError::Serialization(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, CacheError>;

/// Redis cache layer for high-performance queries
#[derive(Clone)]
pub struct Cache {
    conn: ConnectionManager,
    default_ttl: Duration,
}

impl Cache {
    /// Create a new cache instance
    pub async fn new(redis_url: &str, default_ttl_secs: u64) -> Result<Self> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| CacheError::Connection(e.to_string()))?;

        let conn = ConnectionManager::new(client)
            .await
            .map_err(|e| CacheError::Connection(e.to_string()))?;

        Ok(Self {
            conn,
            default_ttl: Duration::from_secs(default_ttl_secs),
        })
    }

    /// Get a value from cache
    pub async fn get<T>(&mut self, key: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let value: Option<String> = self.conn.get(key).await?;

        match value {
            Some(v) => {
                let deserialized: T = serde_json::from_str(&v)?;
                tracing::debug!("Cache HIT: {}", key);
                Ok(Some(deserialized))
            }
            None => {
                tracing::debug!("Cache MISS: {}", key);
                Ok(None)
            }
        }
    }

    /// Set a value in cache with default TTL
    pub async fn set<T>(&mut self, key: &str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.set_with_ttl(key, value, self.default_ttl).await
    }

    /// Set a value in cache with custom TTL
    pub async fn set_with_ttl<T>(&mut self, key: &str, value: &T, ttl: Duration) -> Result<()>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string(value)?;
        self.conn
            .set_ex(key, serialized, ttl.as_secs())
            .await?;
        tracing::debug!("Cache SET: {} (TTL: {}s)", key, ttl.as_secs());
        Ok(())
    }

    /// Delete a value from cache
    pub async fn delete(&mut self, key: &str) -> Result<()> {
        self.conn.del(key).await?;
        tracing::debug!("Cache DELETE: {}", key);
        Ok(())
    }

    /// Delete multiple keys matching a pattern
    pub async fn delete_pattern(&mut self, pattern: &str) -> Result<u64> {
        let keys: Vec<String> = self.conn.keys(pattern).await?;
        if keys.is_empty() {
            return Ok(0);
        }
        let count = keys.len() as u64;
        self.conn.del(&keys).await?;
        tracing::debug!("Cache DELETE_PATTERN: {} ({} keys)", pattern, count);
        Ok(count)
    }

    /// Check if a key exists
    pub async fn exists(&mut self, key: &str) -> Result<bool> {
        let exists: bool = self.conn.exists(key).await?;
        Ok(exists)
    }

    /// Set expiration on a key
    pub async fn expire(&mut self, key: &str, ttl: Duration) -> Result<()> {
        self.conn.expire(key, ttl.as_secs() as i64).await?;
        Ok(())
    }

    /// Get multiple values
    pub async fn mget<T>(&mut self, keys: &[String]) -> Result<Vec<Option<T>>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let values: Vec<Option<String>> = self.conn.mget(keys).await?;

        let mut results = Vec::new();
        for value in values {
            match value {
                Some(v) => {
                    let deserialized: T = serde_json::from_str(&v)?;
                    results.push(Some(deserialized));
                }
                None => results.push(None),
            }
        }

        Ok(results)
    }

    /// Increment a counter
    pub async fn incr(&mut self, key: &str) -> Result<i64> {
        let value: i64 = self.conn.incr(key, 1).await?;
        Ok(value)
    }

    /// Increment a counter with expiration
    pub async fn incr_with_ttl(&mut self, key: &str, ttl: Duration) -> Result<i64> {
        let value: i64 = self.conn.incr(key, 1).await?;
        let _: bool = self.conn.expire(key, ttl.as_secs() as i64).await?;
        Ok(value)
    }

    /// Health check
    pub async fn health_check(&mut self) -> Result<()> {
        let _: String = redis::cmd("PING")
            .query_async(&mut self.conn)
            .await
            .map_err(|e| CacheError::Connection(format!("Health check failed: {}", e)))?;
        Ok(())
    }

    /// Flush all cache (USE WITH CAUTION)
    pub async fn flush_all(&mut self) -> Result<()> {
        let _: () = redis::cmd("FLUSHDB")
            .query_async(&mut self.conn)
            .await?;
        tracing::warn!("Cache FLUSHED");
        Ok(())
    }
}

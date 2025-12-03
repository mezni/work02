use async_trait::async_trait;
use redis::{AsyncCommands, Client};
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::Duration;
use crate::domain::user::User;
use super::error::{InfrastructureError, InfrastructureResult};

#[async_trait]
pub trait Cache: Send + Sync {
    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> InfrastructureResult<Option<T>>;
    async fn set<T: Serialize + Send>(&self, key: &str, value: &T, ttl_seconds: Option<u64>) -> InfrastructureResult<()>;
    async fn delete(&self, key: &str) -> InfrastructureResult<()>;
    async fn exists(&self, key: &str) -> InfrastructureResult<bool>;
    async fn clear(&self) -> InfrastructureResult<()>;
}

pub struct RedisCache {
    client: Client,
    default_ttl: u64,
}

impl RedisCache {
    pub fn new(url: &str, default_ttl: u64) -> InfrastructureResult<Self> {
        let client = Client::open(url)
            .map_err(|e| InfrastructureError::RedisConnection(e.into()))?;
        
        Ok(Self {
            client,
            default_ttl,
        })
    }
    
    pub async fn test_connection(&self) -> InfrastructureResult<()> {
        let mut conn = self.client.get_async_connection().await
            .map_err(|e| InfrastructureError::RedisConnection(e.into()))?;
        
        redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .map_err(|e| InfrastructureError::RedisConnection(e.into()))?;
        
        Ok(())
    }
}

#[async_trait]
impl Cache for RedisCache {
    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> InfrastructureResult<Option<T>> {
        let mut conn = self.client.get_async_connection().await
            .map_err(|e| InfrastructureError::RedisConnection(e.into()))?;
        
        let data: Option<String> = conn.get(key).await
            .map_err(|e| InfrastructureError::RedisConnection(e.into()))?;
        
        match data {
            Some(json) => {
                let value: T = serde_json::from_str(&json)
                    .map_err(|e| InfrastructureError::Serialization(e.into()))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
    
    async fn set<T: Serialize + Send>(&self, key: &str, value: &T, ttl_seconds: Option<u64>) -> InfrastructureResult<()> {
        let mut conn = self.client.get_async_connection().await
            .map_err(|e| InfrastructureError::RedisConnection(e.into()))?;
        
        let json = serde_json::to_string(value)
            .map_err(|e| InfrastructureError::Serialization(e.into()))?;
        
        let ttl = ttl_seconds.unwrap_or(self.default_ttl);
        
        if ttl > 0 {
            redis::pipe()
                .set(key, json)
                .expire(key, ttl as usize)
                .query_async(&mut conn)
                .await
                .map_err(|e| InfrastructureError::RedisConnection(e.into()))?;
        } else {
            conn.set(key, json).await
                .map_err(|e| InfrastructureError::RedisConnection(e.into()))?;
        }
        
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> InfrastructureResult<()> {
        let mut conn = self.client.get_async_connection().await
            .map_err(|e| InfrastructureError::RedisConnection(e.into()))?;
        
        conn.del(key).await
            .map_err(|e| InfrastructureError::RedisConnection(e.into()))?;
        
        Ok(())
    }
    
    async fn exists(&self, key: &str) -> InfrastructureResult<bool> {
        let mut conn = self.client.get_async_connection().await
            .map_err(|e| InfrastructureError::RedisConnection(e.into()))?;
        
        let exists: bool = conn.exists(key).await
            .map_err(|e| InfrastructureError::RedisConnection(e.into()))?;
        
        Ok(exists)
    }
    
    async fn clear(&self) -> InfrastructureResult<()> {
        let mut conn = self.client.get_async_connection().await
            .map_err(|e| InfrastructureError::RedisConnection(e.into()))?;
        
        redis::cmd("FLUSHDB")
            .query_async(&mut conn)
            .await
            .map_err(|e| InfrastructureError::RedisConnection(e.into()))?;
        
        Ok(())
    }
}

pub struct InMemoryCache {
    store: Arc<RwLock<HashMap<String, (String, Option<tokio::time::Instant>)>>>,
    default_ttl: Duration,
}

impl InMemoryCache {
    pub fn new(default_ttl_seconds: u64) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            default_ttl: Duration::from_secs(default_ttl_seconds),
        }
    }
    
    async fn cleanup_expired(&self) {
        let mut store = self.store.write().await;
        let now = tokio::time::Instant::now();
        
        store.retain(|_, (_, expiration)| {
            match expiration {
                Some(exp) => *exp > now,
                None => true,
            }
        });
    }
}

#[async_trait]
impl Cache for InMemoryCache {
    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> InfrastructureResult<Option<T>> {
        self.cleanup_expired().await;
        
        let store = self.store.read().await;
        
        if let Some((json, expiration)) = store.get(key) {
            if let Some(exp) = expiration {
                if *exp <= tokio::time::Instant::now() {
                    return Ok(None);
                }
            }
            
            let value: T = serde_json::from_str(json)
                .map_err(|e| InfrastructureError::Serialization(e.into()))?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
    
    async fn set<T: Serialize + Send>(&self, key: &str, value: &T, ttl_seconds: Option<u64>) -> InfrastructureResult<()> {
        let json = serde_json::to_string(value)
            .map_err(|e| InfrastructureError::Serialization(e.into()))?;
        
        let expiration = ttl_seconds.map(|ttl| {
            tokio::time::Instant::now() + Duration::from_secs(ttl)
        });
        
        let mut store = self.store.write().await;
        store.insert(key.to_string(), (json, expiration));
        
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> InfrastructureResult<()> {
        let mut store = self.store.write().await;
        store.remove(key);
        Ok(())
    }
    
    async fn exists(&self, key: &str) -> InfrastructureResult<bool> {
        self.cleanup_expired().await;
        
        let store = self.store.read().await;
        Ok(store.contains_key(key))
    }
    
    async fn clear(&self) -> InfrastructureResult<()> {
        let mut store = self.store.write().await;
        store.clear();
        Ok(())
    }
}

// User-specific cache operations
#[async_trait]
pub trait UserCache: Cache {
    async fn get_user(&self, user_id: &str) -> InfrastructureResult<Option<User>> {
        let key = format!("user:{}", user_id);
        self.get(&key).await
    }
    
    async fn set_user(&self, user: &User, ttl_seconds: Option<u64>) -> InfrastructureResult<()> {
        let key = format!("user:{}", user.id);
        self.set(&key, user, ttl_seconds).await
    }
    
    async fn delete_user(&self, user_id: &str) -> InfrastructureResult<()> {
        let key = format!("user:{}", user_id);
        self.delete(&key).await
    }
    
    async fn get_user_by_email(&self, email: &str) -> InfrastructureResult<Option<User>> {
        let key = format!("user:email:{}", email);
        self.get(&key).await
    }
    
    async fn set_user_by_email(&self, email: &str, user: &User, ttl_seconds: Option<u64>) -> InfrastructureResult<()> {
        let key = format!("user:email:{}", email);
        self.set(&key, user, ttl_seconds).await
    }
}

#[async_trait]
impl<T: Cache> UserCache for T {}
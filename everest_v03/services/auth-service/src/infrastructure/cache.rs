use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Simple in-memory cache with TTL support
pub struct Cache<K, V> {
    store: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    default_ttl: Duration,
}

struct CacheEntry<V> {
    value: V,
    expires_at: Instant,
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            default_ttl,
        }
    }

    pub fn with_capacity(capacity: usize, default_ttl: Duration) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::with_capacity(capacity))),
            default_ttl,
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let store = self.store.read().ok()?;
        
        if let Some(entry) = store.get(key) {
            if Instant::now() < entry.expires_at {
                return Some(entry.value.clone());
            }
        }
        
        None
    }

    pub fn set(&self, key: K, value: V) {
        self.set_with_ttl(key, value, self.default_ttl);
    }

    pub fn set_with_ttl(&self, key: K, value: V, ttl: Duration) {
        if let Ok(mut store) = self.store.write() {
            let entry = CacheEntry {
                value,
                expires_at: Instant::now() + ttl,
            };
            store.insert(key, entry);
        }
    }

    pub fn remove(&self, key: &K) -> Option<V> {
        if let Ok(mut store) = self.store.write() {
            store.remove(key).map(|entry| entry.value)
        } else {
            None
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn clear(&self) {
        if let Ok(mut store) = self.store.write() {
            store.clear();
        }
    }

    pub fn cleanup_expired(&self) {
        if let Ok(mut store) = self.store.write() {
            let now = Instant::now();
            store.retain(|_, entry| now < entry.expires_at);
        }
    }

    pub fn len(&self) -> usize {
        if let Ok(store) = self.store.read() {
            store.len()
        } else {
            0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K, V> Clone for Cache<K, V> {
    fn clone(&self) -> Self {
        Self {
            store: Arc::clone(&self.store),
            default_ttl: self.default_ttl,
        }
    }
}

/// User-specific cache for frequently accessed data
pub type UserCache = Cache<String, crate::domain::user::User>;

impl UserCache {
    pub fn new_with_defaults() -> Self {
        // Default TTL of 5 minutes for user data
        Cache::new(Duration::from_secs(300))
    }
}

/// Token cache for JWT validation results
#[derive(Clone)]
pub struct TokenCacheEntry {
    pub user_id: String,
    pub role: String,
    pub email: String,
}

pub type TokenCache = Cache<String, TokenCacheEntry>;

impl TokenCache {
    pub fn new_with_defaults() -> Self {
        // Default TTL of 15 minutes for tokens
        Cache::new(Duration::from_secs(900))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_cache_set_get() {
        let cache: Cache<String, i32> = Cache::new(Duration::from_secs(60));
        
        cache.set("key1".to_string(), 42);
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        assert_eq!(cache.get(&"key2".to_string()), None);
    }

    #[test]
    fn test_cache_expiration() {
        let cache: Cache<String, i32> = Cache::new(Duration::from_millis(100));
        
        cache.set("key1".to_string(), 42);
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        
        // Wait for expiration
        thread::sleep(Duration::from_millis(150));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_remove() {
        let cache: Cache<String, i32> = Cache::new(Duration::from_secs(60));
        
        cache.set("key1".to_string(), 42);
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        
        let removed = cache.remove(&"key1".to_string());
        assert_eq!(removed, Some(42));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_clear() {
        let cache: Cache<String, i32> = Cache::new(Duration::from_secs(60));
        
        cache.set("key1".to_string(), 42);
        cache.set("key2".to_string(), 43);
        assert_eq!(cache.len(), 2);
        
        cache.clear();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_cache_cleanup_expired() {
        let cache: Cache<String, i32> = Cache::new(Duration::from_millis(100));
        
        cache.set("key1".to_string(), 42);
        cache.set_with_ttl("key2".to_string(), 43, Duration::from_secs(60));
        
        thread::sleep(Duration::from_millis(150));
        
        cache.cleanup_expired();
        assert_eq!(cache.get(&"key1".to_string()), None);
        assert_eq!(cache.get(&"key2".to_string()), Some(43));
    }

    #[test]
    fn test_cache_clone() {
        let cache1: Cache<String, i32> = Cache::new(Duration::from_secs(60));
        cache1.set("key1".to_string(), 42);
        
        let cache2 = cache1.clone();
        assert_eq!(cache2.get(&"key1".to_string()), Some(42));
        
        cache2.set("key2".to_string(), 43);
        assert_eq!(cache1.get(&"key2".to_string()), Some(43));
    }
}
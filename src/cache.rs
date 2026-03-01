use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::time::Duration;

#[derive(Clone)]
pub struct AppCache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    inner: Cache<K, V>,
}

impl<K, V> AppCache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new(max_capacity: u64, ttl_secs: u64) -> Self {
        Self {
            inner: Cache::builder()
                .max_capacity(max_capacity)
                .time_to_live(Duration::from_secs(ttl_secs))
                .build(),
        }
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        self.inner.get(key).await
    }

    pub async fn insert(&self, key: K, value: V) {
        self.inner.insert(key, value).await;
    }

    pub async fn invalidate(&self, key: &K) {
        self.inner.invalidate(key).await;
    }

    pub async fn _invalidate_all(&self) {
        self.inner.invalidate_all();
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct _CacheEntry<T> {
    pub data: T,
    pub cached_at: i64,
}

impl<T> _CacheEntry<T> {
    pub fn _new(data: T) -> Self {
        Self {
            data,
            cached_at: chrono::Utc::now().timestamp(),
        }
    }
}

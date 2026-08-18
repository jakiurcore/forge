//! A read/write-locked key-value cache for read-heavy or write-heavy workloads.

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

/// Simple concurrent cache using `RwLock`.
#[derive(Debug, Clone)]
pub struct RwLockedCache<K, V> {
    inner: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> Default for RwLockedCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> RwLockedCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Create an empty cache.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Insert a key-value pair.
    pub fn insert(&self, key: K, value: V) {
        let mut guard = self.inner.write().unwrap();
        guard.insert(key, value);
    }

    /// Read a value by key.
    pub fn get(&self, key: &K) -> Option<V> {
        let guard = self.inner.read().unwrap();
        guard.get(key).cloned()
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn insert_and_get() {
        let cache = RwLockedCache::new();
        cache.insert("key", 42);
        assert_eq!(cache.get(&"key"), Some(42));
    }

    #[test]
    fn concurrent_reads_and_writes() {
        let cache = RwLockedCache::new();
        let mut handles = Vec::new();

        for i in 0..10 {
            let c = cache.clone();
            handles.push(thread::spawn(move || {
                c.insert(i, i * 2);
                c.get(&i)
            }));
        }

        for h in handles {
            let result = h.join().unwrap();
            assert!(result.is_some());
        }

        assert_eq!(cache.len(), 10);
    }
}

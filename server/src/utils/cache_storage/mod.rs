use std::{hash::Hash, sync::Arc};
use dashmap::{DashMap, mapref::one::MappedRef};
use crate::utils::cache_storage::policies::CachePolicy;

pub mod policies;

pub struct CacheStorage<P, K, V>
where 
    K: Sync + Send + Hash + Eq,
    V: Sync + Send,
    P: CachePolicy
{
    dash_map: Arc<DashMap<K, P::ValueScheme<V>>>
}

impl<P, K, V> Clone for CacheStorage<P, K, V>
where
    P: CachePolicy,
    K: Sync + Send + Hash + Eq,
    V: Sync + Send
{
    fn clone(&self) -> Self {
        Self {
            dash_map: self.dash_map.clone()
        }
    }
}

impl<P, K, V> CacheStorage<P, K, V>
where
    P: CachePolicy,
    K: Sync + Send + Hash + Eq,
    V: Sync + Send
{
    pub fn new() -> Self {
        Self {
            dash_map: Arc::new(DashMap::new()),
        }
    }

    pub fn insert(&self, key: K, value: V, params: P::ValueParams) -> Option<V> {
        P::insert(&self.dash_map, key, value, params)
    }

    pub fn remove(&self, key: &K) -> Option<V> {
        P::remove(&self.dash_map, key)
    }

    pub fn get(&self, key: &K) -> Option<MappedRef<'_, K, P::ValueScheme<V>, V>> {
        P::get(&self.dash_map, key)
    }

    fn cleaning(&self) {
        P::cleaning(&self.dash_map);
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::{self, sleep}, time::Duration};

    use crate::utils::cache_storage::{CacheStorage, policies::TTL};

    #[test]
    fn test_ttl_storage() {
        let storage: CacheStorage<TTL, String, String> = CacheStorage::new();

        storage.insert("Hello".to_string(), "World".to_string(), TTL::after_ms(100));

        // It's thread safe
        // You can use .clone() method to share the storage between threads
        let first_thread = {
            let storage_clone = storage.clone();

            thread::spawn(move || {
                let guard = storage_clone.get(&"Hello".to_string()).unwrap();
                let value = guard.value();

                assert_eq!(value, "World");
            })
        };


        let second_thread = {
            let storage_clone = storage.clone();

            thread::spawn(move || {
                sleep(Duration::from_millis(150));

                let guard = storage_clone.get(&"Hello".to_string());
                assert!(guard.is_none());
            })
        };

        let _ = first_thread.join().unwrap();
        let _ = second_thread.join().unwrap();
    }
}


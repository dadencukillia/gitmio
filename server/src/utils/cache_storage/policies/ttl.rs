use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::cache_storage::policies::CachePolicy;

pub struct TTL;

pub struct TTLValueScheme<V: Sync + Send> {
    pub value: V,
    pub expire_at: u128
}

impl CachePolicy for TTL {
    type ValueScheme<V: Sync + Send> = TTLValueScheme<V>;
    type ValueParams = u128;

    fn wrap_value<V: Sync + Send>(value: V, params: Self::ValueParams) -> Self::ValueScheme<V> {
        TTLValueScheme {
            value,
            expire_at: params
        }
    }

    fn unwrap_value_ref<'a, V: Sync + Send>(storage_value: &'a Self::ValueScheme<V>) -> &'a V {
        &storage_value.value
    }

    fn unwrap_value<V: Sync + Send>(storage_value: Self::ValueScheme<V>) -> V {
        storage_value.value
    }

    fn validate_storage_value<V: Sync + Send>(storage_value: &Self::ValueScheme<V>) -> bool {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() < storage_value.expire_at
    }

    fn cleaning<K, V>(dash_map: &dashmap::DashMap<K, Self::ValueScheme<V>>) 
    where
        K: Sync + Send + std::hash::Hash + Eq,
        V: Sync + Send 
    {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        dash_map.retain(|_k, v| {
            now < v.expire_at
        });
    }
}

impl TTL {
    pub fn from_timestamp_ms(ms: u128) -> u128 {
        ms
    }

    pub fn after_ms(ms: u128) -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() + ms
    }
}


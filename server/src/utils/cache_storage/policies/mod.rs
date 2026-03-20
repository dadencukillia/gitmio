mod ttl;

pub use ttl::*;

use std::{hash::Hash, sync::Arc};
use dashmap::{DashMap, mapref::one::MappedRef};

pub trait CachePolicy {
    type ValueScheme<V>: Sync + Send
    where
        V: Sync + Send;
    type ValueParams;

    fn wrap_value<V: Sync + Send>(value: V, params: Self::ValueParams) -> Self::ValueScheme<V>;
    fn unwrap_value_ref<'a, V: Sync + Send>(storage_value: &'a Self::ValueScheme<V>) -> &'a V;
    fn unwrap_value<V: Sync + Send>(storage_value: Self::ValueScheme<V>) -> V;
    fn validate_storage_value<V: Sync + Send>(storage_value: &Self::ValueScheme<V>) -> bool;

    fn insert<K, V>(dash_map: &DashMap<K, Self::ValueScheme<V>>, key: K, value: V, params: Self::ValueParams) -> Option<V>
    where 
        K: Sync + Send + Hash + Eq,
        V: Sync + Send
    {
        let storage_value = dash_map.insert(key, Self::wrap_value(value, params))?;
        Some(Self::unwrap_value::<V>(storage_value))
    }

    fn remove<K, V>(dash_map: &DashMap<K, Self::ValueScheme<V>>, key: &K) -> Option<V>
    where
        K: Sync + Send + Hash + Eq,
        V: Sync + Send
    {
        let storage_value = dash_map.remove(key)?;
        Some(Self::unwrap_value::<V>(storage_value.1))
    }

    fn get<'a, K, V>(dash_map: &'a DashMap<K, Self::ValueScheme<V>>, key: &K) -> Option<MappedRef<'a, K, Self::ValueScheme<V>, V>>
    where
        K: Sync + Send + Hash + Eq,
        V: Sync + Send
    {
        let guard = dash_map.get(key)?;

        if !Self::validate_storage_value(guard.value()) {
            drop(guard);

            dash_map.remove_if(key, |_k, v| !Self::validate_storage_value(v));

            return None;
        }

        Some(guard.map(|val| Self::unwrap_value_ref(val)))
    }

    fn cleaning<K, V>(dash_map: &DashMap<K, Self::ValueScheme<V>>) 
    where
        K: Sync + Send + Hash + Eq,
        V: Sync + Send
    {
        dash_map.retain(|_, v| {
            Self::validate_storage_value(v)
        });
    }
}


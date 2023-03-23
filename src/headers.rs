use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Borrow, collections::BTreeMap, error::Error};


pub trait Map<'a, K, V>
    where
        K: Serialize + DeserializeOwned + ?Sized,
        V: Serialize + DeserializeOwned,
{
    type Error: Error;
    type Iterator: Iterator<Item = (K, V)>;
    type Keys: Iterator<Item = K>;
    type Values: Iterator<Item = V>;

    fn contains_key(&self, key: &K) -> Result<bool, Self::Error>;
    fn get(&self, key: &K) -> Result<Option<V>, Self::Error>;
    fn get_raw_bytes(&self, key: &K) -> Result<Option<Vec<u8>>, Self::Error>;
    fn get_or_insert_unsafe<F: FnOnce() -> V>(
        &self,
        key: &K,
        default: F,
    ) -> Result<V, Self::Error> {
        self.get(key).and_then(|optv| match optv {
            Some(v) => Ok(v),
            None => {
                self.insert(key, &default())?;
                self.get(key).transpose().expect("default just inserted")
            }
        })
    }
    fn insert(&self, key: &K, value: &V) -> Result<(), Self::Error>;
    fn remove(&self, key: &K) -> Result<(), Self::Error>;
    fn clear(&self) -> Result<(), Self::Error>;
    fn is_empty(&self) -> bool;
    fn iter(&'a self) -> Self::Iterator;
    fn keys(&'a self) -> Self::Keys;
    fn values(&'a self) -> Self::Values;
    fn multi_get<J>(&self, keys: impl IntoIterator<Item = J>) -> Result<Vec<Option<V>>, Self::Error>
        where
            J: Borrow<K>,
    {
        keys.into_iter().map(|key| self.get(key.borrow())).collect()
    }
    fn multi_insert<J, U>(
        &self,
        key_val_pairs: impl IntoIterator<Item = (J, U)>,
    ) -> Result<(), Self::Error>
        where
            J: Borrow<K>,
            U: Borrow<V>,
    {
        key_val_pairs
            .into_iter()
            .try_for_each(|(key, value)| self.insert(key.borrow(), value.borrow()))
    }
    fn multi_remove<J>(&self, keys: impl IntoIterator<Item = J>) -> Result<(), Self::Error>
        where
            J: Borrow<K>,
    {
        keys.into_iter()
            .try_for_each(|key| self.remove(key.borrow()))
    }
    fn try_catch_up_with_primary(&self) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait AsyncMap<'a, K, V>
    where
        K: Serialize + DeserializeOwned + ?Sized + std::marker::Sync,
        V: Serialize + DeserializeOwned + std::marker::Sync + std::marker::Send,
{
    type Error: Error;
    type Iterator: Iterator<Item = (K, V)>;
    type Keys: Iterator<Item = K>;
    type Values: Iterator<Item = V>;
    
    async fn contains_key(&self, key: &K) -> Result<bool, Self::Error>;
    async fn get(&self, key: &K) -> Result<Option<V>, Self::Error>;
    async fn get_raw_bytes(&self, key: &K) -> Result<Option<Vec<u8>>, Self::Error>;
    async fn is_empty(&self) -> bool;
    async fn iter(&'a self) -> Self::Iterator;
    async fn keys(&'a self) -> Self::Keys;
    async fn values(&'a self) -> Self::Values;
    async fn multi_get<J>(
        &self,
        keys: impl IntoIterator<Item = J> + std::marker::Send,
    ) -> Result<Vec<Option<V>>, Self::Error>
        where
            J: Borrow<K>;
    async fn try_catch_up_with_primary(&self) -> Result<(), Self::Error>;
}

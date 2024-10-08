use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    slice::{Iter, IterMut},
};

use async_graphql::{connection::CursorType, OutputType};
use serde::{Deserialize, Serialize};

use crate::graphql::Paginable;

/// An immutable ordered map, iteration order is values comparison order
#[derive(Clone, Serialize, Deserialize)]
pub struct ValueOrdMap<K: Eq + Hash, V: Ord> {
    keys: Vec<K>,
    values: Vec<V>,
    keys_by_hash: HashMap<u64, usize>,
}

impl<K: Eq + Hash, V: Ord> ValueOrdMap<K, V> {
    pub fn empty() -> Self {
        Self {
            keys: vec![],
            values: vec![],
            keys_by_hash: HashMap::new(),
        }
    }

    fn key_hash(key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.keys_by_hash.contains_key(&Self::key_hash(key))
    }

    pub fn get_key_index(&self, key: &K) -> Option<usize> {
        self.keys_by_hash.get(&Self::key_hash(key)).copied()
    }

    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        self.get_key_index(key)
            .map(|index| self.values.get(index).unwrap())
    }

    pub fn keys(&self) -> Iter<K> {
        self.keys.iter()
    }

    pub fn values(&self) -> Iter<V> {
        self.values.iter()
    }

    pub fn values_mut(&mut self) -> IterMut<V> {
        self.values.iter_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.keys().zip(self.values())
    }

    pub fn into_values(self) -> Vec<V> {
        self.values
    }

    pub fn len(&self) -> usize {
        self.keys_by_hash.len()
    }
}

impl<K: Eq + Hash, V: Ord> FromIterator<(K, V)> for ValueOrdMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut from_entries: Vec<_> = iter.into_iter().collect();
        from_entries.sort_by(|(_, a), (_, b)| a.cmp(b));

        let mut keys = Vec::with_capacity(from_entries.len());
        let mut values = Vec::with_capacity(from_entries.len());
        let mut keys_by_hash = HashMap::with_capacity(from_entries.len());

        for (i, (key, value)) in from_entries.into_iter().enumerate() {
            keys_by_hash.insert(Self::key_hash(&key), i);
            keys.push(key);
            values.push(value);
        }

        Self {
            keys,
            values,
            keys_by_hash,
        }
    }
}

// TODO: remove reference?
impl<K: CursorType + Eq + Hash, V: OutputType + Clone + Ord> Paginable for &'_ ValueOrdMap<K, V> {
    type By = K;
    type Item = V;

    fn len(&self) -> usize {
        self.values.len()
    }

    fn find_pos(&self, cursor: &Self::By) -> Option<usize> {
        ValueOrdMap::get_key_index(self, cursor)
    }

    fn iter_ordered(
        &self,
    ) -> impl DoubleEndedIterator<Item = &Self::Item> + ExactSizeIterator<Item = &Self::Item> {
        self.values.iter()
    }
}

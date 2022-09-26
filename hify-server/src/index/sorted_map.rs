use std::{
    collections::{hash_map::Keys, HashMap},
    hash::Hash,
    slice::Iter,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SortedMap<K: Eq + Hash, V: Ord> {
    values: Vec<V>,
    indexes: HashMap<K, usize>,
}

impl<K: Eq + Hash, V: Ord> SortedMap<K, V> {
    pub fn from_vec(mut values: Vec<V>, value_index: impl Fn(&V) -> K) -> Self {
        values.sort();

        let indexes = values
            .iter()
            .enumerate()
            .map(|(i, value)| (value_index(value), i))
            .collect();

        Self { values, indexes }
    }

    pub fn from_hashmap(map: HashMap<K, V>) -> Self {
        let mut entries: Vec<_> = map.into_iter().collect();
        entries.sort_by(|(_, a), (_, b)| a.cmp(b));

        let mut values = Vec::with_capacity(entries.len());
        let mut indexes = HashMap::with_capacity(entries.len());

        for (i, (key, value)) in entries.into_iter().enumerate() {
            values.push(value);
            indexes.insert(key, i);
        }

        Self { values, indexes }
    }

    pub fn empty() -> Self {
        Self {
            values: vec![],
            indexes: HashMap::new(),
        }
    }

    // pub fn contains_key(&self, key: &K) -> bool {
    //     self.indexes.contains_key(key)
    // }

    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        let index = self.indexes.get(key)?;
        Some(self.values.get(*index).unwrap())
    }

    pub fn get_index(&self, key: &K) -> Option<usize> {
        self.indexes.get(key).copied()
    }

    // // TODO: improve by using .rev() on self.indexes instead of building a reversed vec of values
    // pub fn to_vec(self) -> Vec<(K, V)> {
    //     let mut values: Vec<_> = self.values.into_iter().rev().collect();

    //     self.indexes
    //         .into_iter()
    //         .map(|(key, _)| (key, values.pop().unwrap()))
    //         .collect()
    // }

    pub fn keys(&self) -> Keys<K, usize> {
        self.indexes.keys()
    }

    pub fn values(&self) -> Iter<V> {
        self.values.iter()
    }

    pub fn into_values(self) -> Vec<V> {
        self.values
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

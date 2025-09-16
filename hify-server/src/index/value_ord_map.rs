use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    marker::PhantomData,
    slice::Iter,
};

use async_graphql::{OutputType, connection::CursorType};
use serde::{
    Deserialize, Serialize,
    de::{MapAccess, Visitor},
};

use crate::graphql::Paginable;

/// A key-value dictionary, whose iteration order is based on values' ordering
#[derive(Clone)]
pub struct ValueOrdMap<K: Eq + Hash, V: Ord> {
    keys: Vec<K>,
    values: Vec<V>,
    keys_by_hash: HashMap<u64, usize>,
}

impl<K: Eq + Hash, V: Ord> ValueOrdMap<K, V> {
    fn from_sorted(entries: impl ExactSizeIterator<Item = (K, V)>) -> Self {
        let mut keys = Vec::with_capacity(entries.len());
        let mut values = Vec::with_capacity(entries.len());
        let mut keys_by_hash = HashMap::with_capacity(entries.len());

        for (i, (key, value)) in entries.enumerate() {
            keys_by_hash.insert(Self::hash_key(&key), i);
            keys.push(key);
            values.push(value);
        }

        Self {
            keys,
            values,
            keys_by_hash,
        }
    }

    pub fn from_entries(mut entries: Vec<(K, V)>) -> Self {
        entries.sort_by(|(_, a), (_, b)| a.cmp(b));
        Self::from_sorted(entries.into_iter())
    }

    pub fn empty() -> Self {
        Self {
            keys: vec![],
            values: vec![],
            keys_by_hash: HashMap::new(),
        }
    }

    fn hash_key(key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.keys_by_hash.contains_key(&Self::hash_key(key))
    }

    pub fn get_key_index(&self, key: &K) -> Option<usize> {
        self.keys_by_hash.get(&Self::hash_key(key)).copied()
    }

    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        self.get_key_index(key)
            .map(|index| self.values.get(index).unwrap())
    }

    pub fn keys(&self) -> Iter<'_, K> {
        self.keys.iter()
    }

    pub fn values(&self) -> Iter<'_, V> {
        self.values.iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.keys().zip(self.values())
    }

    pub fn len(&self) -> usize {
        self.keys_by_hash.len()
    }
}

impl<K: Eq + Hash, V: Ord> FromIterator<(K, V)> for ValueOrdMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::from_entries(iter.into_iter().collect())
    }
}

impl<K: CursorType + Eq + Hash, V: OutputType + Clone + Ord> Paginable for ValueOrdMap<K, V> {
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

// Manual serialization allows to serialize this as a simple dictionary-like structure
// instead of serializing every field from the [`ValueOrdMap`] struct
impl<K: Eq + Hash + Serialize, V: Ord + Serialize> Serialize for ValueOrdMap<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_map(self.iter())
    }
}

// Manual serialization means we must also use manual deserialization
impl<'de, K: Eq + Hash + Deserialize<'de>, V: Ord + Deserialize<'de>> Deserialize<'de>
    for ValueOrdMap<K, V>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MapVisitor<K, V> {
            marker: PhantomData<(K, V)>,
        }

        impl<'de, K: Eq + Hash + Deserialize<'de>, V: Ord + Deserialize<'de>> Visitor<'de>
            for MapVisitor<K, V>
        {
            type Value = ValueOrdMap<K, V>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map")
            }

            #[inline]
            fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut entries = vec![];

                while let Some(entry) = access.next_entry::<K, V>()? {
                    entries.push(entry);
                }

                Ok(ValueOrdMap::from_entries(entries))
            }
        }

        let visitor = MapVisitor {
            marker: PhantomData,
        };

        deserializer.deserialize_map(visitor)
    }
}

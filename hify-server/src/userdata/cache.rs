use std::collections::HashMap;

use jiff::Zoned;
use serde::{Deserialize, Serialize};

use crate::index::{Index, TrackID};

use super::{
    history::{History, OneListening},
    UserDataConfig,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserDataCache {
    config: UserDataConfig,
    dedup_history: Vec<OneListening>,
    listenings: HashMap<TrackID, u32>,
    listening_durations: HashMap<TrackID, u32>,
    last_listening: HashMap<TrackID, Zoned>,
}

impl UserDataCache {
    pub fn new(history: &History, config: UserDataConfig) -> Self {
        let mut created = Self {
            config,
            listenings: HashMap::new(),
            listening_durations: HashMap::new(),
            last_listening: HashMap::new(),
            dedup_history: vec![],
        };

        for entry in history.entries() {
            created.update_with(entry);
        }

        created
    }

    pub fn update_with(&mut self, entry: &OneListening) {
        *self.listenings.entry(entry.track_id).or_default() += 1;
        *self.listening_durations.entry(entry.track_id).or_default() += entry.duration_s;

        match self
            .dedup_history
            .iter()
            .position(|t| t.track_id == entry.track_id)
        {
            Some(index) => {
                self.dedup_history.remove(index);
            }

            None => {
                if self.dedup_history.len() == self.config.history_cache_capacity {
                    self.dedup_history.pop().unwrap();
                }
            }
        }

        self.last_listening.insert(entry.track_id, entry.at.clone());
        self.dedup_history.insert(0, entry.clone());
    }

    pub fn dedup_history(&self) -> &[OneListening] {
        &self.dedup_history
    }

    pub fn listening_durations(&self) -> &HashMap<TrackID, u32> {
        &self.listening_durations
    }

    pub fn last_listening(&self) -> &HashMap<TrackID, Zoned> {
        &self.last_listening
    }

    pub fn cleanup(&mut self, new_index: &Index) {
        self.dedup_history
            .retain(|listening| new_index.tracks.contains_key(&listening.track_id));

        self.listenings
            .retain(|track_id, _| new_index.tracks.contains_key(track_id));

        self.listening_durations
            .retain(|track_id, _| new_index.tracks.contains_key(track_id));

        self.last_listening
            .retain(|track_id, _| new_index.tracks.contains_key(track_id));
    }
}

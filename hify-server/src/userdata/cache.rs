use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::index::TrackID;

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
}

impl UserDataCache {
    pub fn new(history: &History, config: UserDataConfig) -> Self {
        let mut created = Self {
            config,
            listenings: HashMap::new(),
            listening_durations: HashMap::new(),
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

        self.dedup_history.insert(0, *entry);
    }

    pub fn dedup_history(&self) -> &[OneListening] {
        &self.dedup_history
    }

    pub fn listenings(&self) -> &HashMap<TrackID, u32> {
        &self.listenings
    }

    pub fn listening_durations(&self) -> &HashMap<TrackID, u32> {
        &self.listening_durations
    }
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::index::TrackID;

use super::history::{History, OneListening};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserDataCache {
    pub listenings: HashMap<TrackID, u32>,
    pub listening_durations: HashMap<TrackID, u32>,
}

impl UserDataCache {
    pub fn new(history: &History) -> Self {
        let mut created = Self {
            listenings: HashMap::new(),
            listening_durations: HashMap::new(),
        };

        for entry in history.entries() {
            created.update_with(entry);
        }

        created
    }

    pub fn update_with(&mut self, entry: &OneListening) {
        *self.listenings.entry(entry.track.clone()).or_default() += 1;
        *self
            .listening_durations
            .entry(entry.track.clone())
            .or_default() += entry.duration_s;
    }

    pub fn listenings(&self) -> &HashMap<TrackID, u32> {
        &self.listenings
    }

    pub fn listening_durations(&self) -> &HashMap<TrackID, u32> {
        &self.listening_durations
    }
}

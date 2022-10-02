mod config;

pub use config::UserDataConfig;

use std::collections::{hash_map::Entry, HashMap};

use serde::{Deserialize, Serialize};

use crate::index::TrackID;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserData {
    config: UserDataConfig,
    history: Vec<TrackID>,
    listenings: HashMap<TrackID, u32>,
    listening_durations: HashMap<TrackID, u64>,
}

impl UserData {
    pub fn new(config: UserDataConfig) -> Self {
        Self {
            config,
            history: vec![],
            listenings: HashMap::new(),
            listening_durations: HashMap::new(),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(UserDataConfig::default())
    }
}

pub struct UserDataWrapper {
    inner: UserData,
    on_change: Box<dyn Fn(&UserData) + Send + Sync>,
}

impl UserDataWrapper {
    pub fn new(inner: UserData, on_change: Box<dyn Fn(&UserData) + Send + Sync>) -> Self {
        Self { inner, on_change }
    }

    pub fn history(&self) -> &[TrackID] {
        &self.inner.history
    }

    pub fn listenings(&self) -> &HashMap<TrackID, u32> {
        &self.inner.listenings
    }

    pub fn history_push(&mut self, track_id: TrackID) {
        match self.inner.history.iter().position(|t| *t == track_id) {
            Some(index) => {
                self.inner.history.remove(index);
            }

            None => {
                if self.inner.history.len() == self.inner.config.history_capacity {
                    self.inner.history.pop().unwrap();
                }
            }
        }

        self.inner.history.insert(0, track_id);

        (self.on_change)(&self.inner);
    }

    pub fn log_listening(&mut self, track_id: TrackID, duration_s: u32) {
        if duration_s < self.inner.config.listening_duration_thresold {
            return;
        }

        match self.inner.listenings.entry(track_id.clone()) {
            Entry::Occupied(mut occ) => *occ.get_mut() += 1,
            Entry::Vacant(vac) => {
                vac.insert(1);
            }
        };

        let duration = u64::from(duration_s);

        match self.inner.listening_durations.entry(track_id) {
            Entry::Occupied(mut occ) => *occ.get_mut() += duration,
            Entry::Vacant(vac) => {
                vac.insert(duration);
            }
        };
    }
}

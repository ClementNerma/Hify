mod cache;
mod config;
mod history;

use std::collections::HashMap;

pub use config::UserDataConfig;
pub use history::OneListening;

use serde::{Deserialize, Serialize};

use crate::index::{Index, Rating, TrackID};

use self::{cache::UserDataCache, history::History};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserData {
    config: UserDataConfig,
    history: History,
    track_ratings: HashMap<TrackID, Rating>,
}

impl UserData {
    pub fn new(config: UserDataConfig) -> Self {
        Self {
            config,
            history: History::new(),
            track_ratings: HashMap::new(),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(UserDataConfig::default())
    }
}

pub struct UserDataWrapper {
    inner: UserData,
    cache: UserDataCache,
    on_change: Box<dyn Fn(&UserData) + Send + Sync>,
}

impl UserDataWrapper {
    pub fn new(inner: UserData, on_change: Box<dyn Fn(&UserData) + Send + Sync>) -> Self {
        Self {
            cache: UserDataCache::new(&inner.history, inner.config),
            inner,
            on_change,
        }
    }

    pub fn cache(&self) -> &UserDataCache {
        &self.cache
    }

    pub fn history(&self) -> &History {
        &self.inner.history
    }

    pub fn track_ratings(&self) -> &HashMap<TrackID, Rating> {
        &self.inner.track_ratings
    }

    pub fn set_track_rating(&mut self, track_id: TrackID, rating: Rating) {
        self.inner.track_ratings.insert(track_id, rating);

        (self.on_change)(&self.inner);
    }

    pub fn remove_track_rating(&mut self, track_id: &TrackID) {
        self.inner.track_ratings.remove(track_id);
    }

    pub fn log_listening(&mut self, entry: OneListening) {
        if entry.duration_s < self.inner.config.listening_duration_thresold {
            return;
        }

        self.cache.update_with(&entry);
        self.inner.history.push(entry);

        (self.on_change)(&self.inner);
    }

    pub fn cleanup(&mut self, new_index: &Index) {
        self.inner.history.cleanup(new_index);
        self.cache.cleanup(new_index);

        (self.on_change)(&self.inner);
    }
}

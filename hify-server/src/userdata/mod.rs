mod cache;
mod config;
mod history;

pub use config::UserDataConfig;
pub use history::OneListening;

use serde::{Deserialize, Serialize};

use self::{cache::UserDataCache, history::History};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserData {
    config: UserDataConfig,
    history: History,
}

impl UserData {
    pub fn new(config: UserDataConfig) -> Self {
        Self {
            config,
            history: History::new(),
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
            cache: UserDataCache::new(&inner.history),
            inner,
            on_change,
        }
    }

    pub fn history(&self) -> &History {
        &self.inner.history
    }

    pub fn cache(&self) -> &UserDataCache {
        &self.cache
    }

    pub fn log_listening(&mut self, entry: OneListening) {
        if entry.duration_s < self.inner.config.listening_duration_thresold {
            return;
        }

        self.cache.update_with(&entry);
        self.inner.history.push(entry);

        (self.on_change)(&self.inner);
    }
}

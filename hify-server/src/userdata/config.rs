use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct UserDataConfig {
    pub listening_duration_thresold: u32,
    pub history_cache_capacity: usize,
}

impl Default for UserDataConfig {
    fn default() -> Self {
        Self {
            listening_duration_thresold: 10,
            history_cache_capacity: 1000,
        }
    }
}

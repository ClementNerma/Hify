use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserDataConfig {
    pub history_capacity: usize,
    pub listening_duration_thresold: u32,
}

impl Default for UserDataConfig {
    fn default() -> Self {
        Self {
            history_capacity: 1000,
            listening_duration_thresold: 10,
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserDataConfig {
    pub listening_duration_thresold: u32,
}

impl Default for UserDataConfig {
    fn default() -> Self {
        Self {
            listening_duration_thresold: 10,
        }
    }
}

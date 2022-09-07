use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {}

impl UserData {
    pub fn new() -> Self {
        Self {}
    }
}

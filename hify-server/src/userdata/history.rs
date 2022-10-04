use std::time::{SystemTime, UNIX_EPOCH};

use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

use crate::index::TrackID;

#[derive(Clone, Serialize, Deserialize)]
pub struct History(Vec<OneListening>);

impl History {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn entries(&self) -> &[OneListening] {
        &self.0
    }

    pub fn push(&mut self, entry: OneListening) {
        self.0.push(entry);
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, InputObject)]
pub struct OneListening {
    at: u64,
    pub track_id: TrackID,
    pub duration_s: u32,
}

impl OneListening {
    pub fn new_now(track_id: TrackID, duration_s: u32) -> Self {
        Self {
            at: Self::now(),
            track_id,
            duration_s,
        }
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get time since Unix' EPOCH")
            .as_secs()
    }

    pub fn at(self) -> u64 {
        self.at
    }
}

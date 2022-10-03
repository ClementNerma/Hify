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

#[derive(Clone, Serialize, Deserialize, InputObject)]
pub struct OneListening {
    pub at: u64,
    pub track_id: TrackID,
    pub duration_s: u32,
}

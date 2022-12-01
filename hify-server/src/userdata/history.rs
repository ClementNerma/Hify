use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    index::{Index, TrackID},
    library::time::get_now,
};

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

    pub fn cleanup(&mut self, new_index: &Index) {
        self.0
            .retain(|track| new_index.tracks.contains_key(&track.track_id));
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct OneListening {
    pub at: OffsetDateTime,
    pub track_id: TrackID,
    pub duration_s: u32,
}

impl OneListening {
    pub fn new_now(track_id: TrackID, duration_s: u32) -> Self {
        Self {
            at: get_now(),
            track_id,
            duration_s,
        }
    }
}

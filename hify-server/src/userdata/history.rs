use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    helpers::time::get_now,
    index::{Index, TrackID},
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

    pub fn is_overlapping_prev(&self, other: OneListening) -> Option<time::Duration> {
        let against = other.at + (time::Duration::SECOND * other.duration_s);

        if self.at < against && against - self.at > time::Duration::SECOND {
            Some(against - self.at)
        } else {
            None
        }
    }
}

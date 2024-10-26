use jiff::{SignedDuration, Timestamp};
use serde::{Deserialize, Serialize};

use crate::index::{Index, TrackID};

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
    pub at: Timestamp,
    pub track_id: TrackID,
    pub duration_s: u32,
}

impl OneListening {
    pub fn new_now(track_id: TrackID, duration_s: u32) -> Self {
        Self {
            at: Timestamp::now(),
            track_id,
            duration_s,
        }
    }

    pub fn is_overlapping_prev(&self, prev: &OneListening) -> Option<SignedDuration> {
        assert!(self.at >= prev.at);

        let against = prev.at + SignedDuration::from_secs(prev.duration_s.into());

        if self.at < against {
            Some(against.duration_since(self.at))
        } else {
            None
        }
    }
}

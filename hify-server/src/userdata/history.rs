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
    pub track_id: TrackID,
    pub ended_at: Timestamp,
    pub duration_s: u32,
}

impl OneListening {
    pub fn new_now(track_id: TrackID, duration_s: u32) -> Self {
        Self {
            duration_s,
            ended_at: Timestamp::now(),
            track_id,
        }
    }

    pub fn is_overlapping_prev(&self, prev: &OneListening) -> Option<SignedDuration> {
        assert!(self.ended_at >= prev.ended_at);

        let curr_start = self.ended_at - SignedDuration::from_secs(self.duration_s.into());

        let overlap = prev.ended_at.duration_since(curr_start);

        if overlap > SignedDuration::from_secs(1) {
            Some(overlap)
        } else {
            None
        }
    }
}

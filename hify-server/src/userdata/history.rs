use jiff::{Span, Zoned};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneListening {
    pub at: Zoned,
    pub track_id: TrackID,
    pub duration_s: u32,
}

impl OneListening {
    pub fn new_now(track_id: TrackID, duration_s: u32) -> Self {
        Self {
            at: Zoned::now(),
            track_id,
            duration_s,
        }
    }

    pub fn is_overlapping_prev(&self, prev: &OneListening) -> Option<Span> {
        assert!(self.at >= prev.at);

        let against = prev
            .at
            .checked_add(Span::new().seconds(prev.duration_s))
            .unwrap();

        if self.at < against {
            Some(against.since(&self.at).unwrap())
        } else {
            None
        }
    }
}

use jiff::Zoned;
use rand::random;
use serde::{Deserialize, Serialize};

use crate::{
    define_id_type,
    index::{Index, TrackID},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Mix {
    id: MixID,
    last_use: Zoned,
    selection: Vec<TrackID>,
}

impl Mix {
    pub fn new(selection: Vec<TrackID>) -> Self {
        Self {
            id: MixID(random()),
            selection,
            last_use: Zoned::now(),
        }
    }

    pub fn id(&self) -> MixID {
        self.id
    }

    pub fn next_tracks<T>(&mut self, max_tracks: usize, mapper: impl Fn(TrackID) -> T) -> Vec<T> {
        self.last_use = Zoned::now();
        self.selection
            .drain(..max_tracks.min(self.selection.len()))
            .map(mapper)
            .collect()
    }

    pub fn cleanup(&mut self, new_index: &Index) {
        self.selection
            .retain(|track_id| new_index.tracks.contains_key(track_id));
    }
}

define_id_type!(MixID);

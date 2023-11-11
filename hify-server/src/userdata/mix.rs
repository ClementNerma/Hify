use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    define_id_type,
    helpers::time::get_now,
    index::{Index, TrackID},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Mix {
    id: MixID,
    last_use: OffsetDateTime,
    selection: Vec<TrackID>,
}

impl Mix {
    pub fn new(selection: Vec<TrackID>) -> Self {
        Self {
            id: MixID(thread_rng().gen()),
            selection,
            last_use: get_now(),
        }
    }

    pub fn id(&self) -> MixID {
        self.id
    }

    // pub fn last_use(&self) -> OffsetDateTime {
    // TODO
    //     self.last_use
    // }

    pub fn next_tracks<T>(&mut self, max_tracks: usize, mapper: impl Fn(TrackID) -> T) -> Vec<T> {
        self.last_use = get_now();
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

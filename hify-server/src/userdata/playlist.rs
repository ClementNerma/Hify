use async_graphql::SimpleObject;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    define_id_type,
    index::{Index, TrackID},
    utils::time::get_now,
};

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Playlist {
    pub id: PlaylistID,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub last_updated_at: OffsetDateTime,

    #[graphql(skip)]
    pub tracks: Vec<TrackID>,
}

impl Playlist {
    pub fn new(name: String) -> Self {
        let now = get_now();

        Self {
            id: PlaylistID(thread_rng().gen()),
            name,
            created_at: now,
            last_updated_at: now,
            tracks: vec![],
        }
    }

    pub fn cleanup(&mut self, new_index: &Index) {
        self.tracks
            .retain(|track_id| new_index.tracks.contains_key(track_id));
    }
}

define_id_type!(PlaylistID);

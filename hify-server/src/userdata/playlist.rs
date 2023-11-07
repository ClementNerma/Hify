use async_graphql::{InputObject, OneofObject, SimpleObject};
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

    pub fn edit(&mut self, action: PlaylistTracksAction) -> Result<(), &'static str> {
        match action {
            PlaylistTracksAction::Add(PlaylistAddTracks { tracks, position }) => {
                if matches!(position, Some(position) if position > self.tracks.len()) {
                    return Err("Provided position is out-of-bounds");
                }

                let position = position.unwrap_or(self.tracks.len());

                self.tracks.splice(position..position, tracks);
            }

            PlaylistTracksAction::Remove(PlaylistRemoveTracks { from, len }) => {
                if from >= self.tracks.len() {
                    return Err("Provided 'from' position is out-of-bounds");
                }

                if from + len > self.tracks.len() {
                    return Err(
                        "Provided 'from' position + length exceeds the playlist's number of tracks",
                    );
                }

                self.tracks.splice(from..from + len, std::iter::empty());
            }

            PlaylistTracksAction::Move(PlaylistMoveTracks {
                from,
                len,
                new_position,
            }) => {
                if from >= self.tracks.len() {
                    return Err("Provided 'from' position is out-of-bounds");
                }

                if from + len > self.tracks.len() {
                    return Err(
                        "Provided 'from' position + length exceeds the playlist's number of tracks",
                    );
                }

                if new_position >= self.tracks.len() {
                    return Err("Provided 'new' position is out-of-bounds");
                }

                if new_position + len > self.tracks.len() {
                    return Err("Provided 'new' position + length is out-of-bounds");
                }

                let moved = self.tracks[new_position..new_position + len].to_vec();

                self.tracks.copy_within(from..from + len, new_position);
                self.tracks[from..from + len].copy_from_slice(&moved);
            }
        }

        Ok(())
    }

    pub fn cleanup(&mut self, new_index: &Index) {
        self.tracks
            .retain(|track_id| new_index.tracks.contains_key(track_id));
    }
}

define_id_type!(PlaylistID);

#[derive(OneofObject)]
pub enum PlaylistTracksAction {
    Add(PlaylistAddTracks),
    Remove(PlaylistRemoveTracks),
    Move(PlaylistMoveTracks),
}

#[derive(InputObject)]
pub struct PlaylistAddTracks {
    tracks: Vec<TrackID>,
    position: Option<usize>,
}

#[derive(InputObject)]
pub struct PlaylistRemoveTracks {
    from: usize,
    len: usize,
}

#[derive(InputObject)]
pub struct PlaylistMoveTracks {
    from: usize,
    len: usize,
    new_position: usize,
}

use std::collections::HashSet;

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
    pub entries: Vec<PlaylistEntry>,
}

impl Playlist {
    pub fn new(name: String, tracks: Vec<TrackID>) -> Self {
        let now = get_now();

        Self {
            id: PlaylistID(thread_rng().gen()),
            name,
            created_at: now,
            last_updated_at: now,
            entries: tracks.into_iter().map(PlaylistEntry::new).collect(),
        }
    }

    pub fn edit(&mut self, action: PlaylistTracksAction) -> Result<(), &'static str> {
        match action {
            PlaylistTracksAction::Add(PlaylistAddTracks { tracks, position }) => {
                if matches!(position, Some(position) if position > self.entries.len()) {
                    return Err("Provided position is out-of-bounds");
                }

                let position = position.unwrap_or(self.entries.len());

                self.entries.splice(
                    position..position,
                    tracks.into_iter().map(PlaylistEntry::new),
                );
            }

            PlaylistTracksAction::Remove(PlaylistRemoveTracks { entries }) => {
                self.entries.retain(|entry| !entries.contains(&entry.id));
            }

            PlaylistTracksAction::Move(PlaylistMoveTracks { entries, move_at }) => {
                if move_at >= self.entries.len() {
                    return Err("Provided position is out-of-bounds");
                }

                if move_at + entries.len() > self.entries.len() {
                    return Err("Provided position + length is out-of-bounds");
                }

                if entries.is_empty() {
                    return Err("Please provide at least one entry to move");
                }

                let positions = entries
                    .into_iter()
                    .map(|entry_id| {
                        match self.entries.iter().position(|entry| entry.id == entry_id) {
                            Some(entry) => Ok(entry),
                            None => Err("One of the provided tracks was not found in the playlist"),
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                if positions
                    .iter()
                    .enumerate()
                    .skip(1)
                    .any(|(i, pos)| *pos != positions[i - 1] + 1)
                {
                    return Err("Position of the provided tracks is not consecutive");
                }

                let len = positions.len();
                let from = positions.into_iter().min().unwrap();

                let moved = self.entries[move_at..move_at + len].to_vec();

                self.entries.copy_within(from..from + len, move_at);
                self.entries[from..from + len].copy_from_slice(&moved);
            }
        }

        Ok(())
    }

    pub fn cleanup(&mut self, new_index: &Index) {
        self.entries
            .retain(|entry| new_index.tracks.contains_key(&entry.track_id));
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct PlaylistEntry {
    id: PlaylistEntryID,

    #[graphql(skip)]
    pub track_id: TrackID,
}

impl PlaylistEntry {
    fn new(track_id: TrackID) -> Self {
        Self {
            id: PlaylistEntryID(thread_rng().gen()),
            track_id,
        }
    }
}

define_id_type!(PlaylistID, PlaylistEntryID);

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
    entries: HashSet<PlaylistEntryID>,
}

#[derive(InputObject)]
pub struct PlaylistMoveTracks {
    entries: Vec<PlaylistEntryID>,
    move_at: usize,
}

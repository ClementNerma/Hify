use std::collections::HashSet;

use async_graphql::{InputObject, OneofObject, SimpleObject};
use jiff::Zoned;
use rand::random;
use serde::{Deserialize, Serialize};

use crate::{
    define_id_type,
    index::{Index, TrackID},
};

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
pub struct Playlist {
    pub id: PlaylistID,
    pub name: String,

    #[graphql(skip)] // TODO
    pub created_at: Zoned,
    #[graphql(skip)] // TODO
    pub last_updated_at: Zoned,
    #[graphql(skip)]
    pub entries: Vec<PlaylistEntry>,
}

impl Playlist {
    pub fn new(name: String, tracks: Vec<TrackID>) -> Self {
        let now = Zoned::now();

        Self {
            id: PlaylistID(random()),
            name,
            created_at: now.clone(),
            last_updated_at: now,
            entries: tracks.into_iter().map(PlaylistEntry::new).collect(),
        }
    }

    pub fn edit(&mut self, action: PlaylistEditAction) -> Result<(), &'static str> {
        match action {
            PlaylistEditAction::Add(PlaylistAddTracks { tracks, position }) => {
                if matches!(position, Some(position) if position > self.entries.len()) {
                    return Err("Provided position is out-of-bounds");
                }

                let position = position.unwrap_or(self.entries.len());

                self.entries.splice(
                    position..position,
                    tracks.into_iter().map(PlaylistEntry::new),
                );
            }

            PlaylistEditAction::Remove(PlaylistRemoveTracks { entries }) => {
                self.entries.retain(|entry| !entries.contains(&entry.id));
            }

            PlaylistEditAction::Move(PlaylistMoveTracks { entries, put_after }) => {
                if entries.is_empty() {
                    return Err("Please provide at least one entry to move");
                }

                let move_at = match put_after {
                    None => 0,
                    Some(put_after) => {
                        self.entries
                            .iter()
                            .position(|entry| entry.id == put_after)
                            .ok_or("Provided reference entry ID was not found in playlist")?
                            + 1
                    }
                };

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
    pub id: PlaylistEntryID,

    #[graphql(skip)]
    pub track_id: TrackID,
}

impl PlaylistEntry {
    fn new(track_id: TrackID) -> Self {
        Self {
            id: PlaylistEntryID(random()),
            track_id,
        }
    }
}

define_id_type!(PlaylistID, PlaylistEntryID);

#[derive(OneofObject)]
pub enum PlaylistEditAction {
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
    put_after: Option<PlaylistEntryID>,
}

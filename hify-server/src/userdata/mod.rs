mod cache;
mod config;
mod history;
mod mix;
mod playlist;
mod wrapper;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::index::{Rating, TrackID};

pub use self::{
    config::UserDataConfig,
    history::OneListening,
    mix::{Mix, MixID},
    playlist::{Playlist, PlaylistEditAction, PlaylistEntry, PlaylistEntryID, PlaylistID},
    wrapper::UserDataWrapper,
};

use self::history::History;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserData {
    config: UserDataConfig,
    history: History,
    track_ratings: HashMap<TrackID, Rating>,
    playlists: HashMap<PlaylistID, Playlist>,
    mixes: HashMap<MixID, Mix>,
}

impl UserData {
    pub fn new(config: UserDataConfig) -> Self {
        Self {
            config,
            history: History::new(),
            track_ratings: HashMap::new(),
            playlists: HashMap::new(),
            mixes: HashMap::new(),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(UserDataConfig::default())
    }
}

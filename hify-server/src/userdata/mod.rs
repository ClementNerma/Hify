mod cache;
mod config;
mod history;
mod playlist;
mod wrapper;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::index::{Rating, TrackID};

pub use self::{
    config::UserDataConfig,
    history::OneListening,
    playlist::{Playlist, PlaylistEntry, PlaylistID, PlaylistTracksAction},
    wrapper::UserDataWrapper,
};

use self::history::History;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserData {
    config: UserDataConfig,
    history: History,
    track_ratings: HashMap<TrackID, Rating>,
    playlists: HashMap<PlaylistID, Playlist>,
}

impl UserData {
    pub fn new(config: UserDataConfig) -> Self {
        Self {
            config,
            history: History::new(),
            track_ratings: HashMap::new(),
            playlists: HashMap::new(),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(UserDataConfig::default())
    }
}

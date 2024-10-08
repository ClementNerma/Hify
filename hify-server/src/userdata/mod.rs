mod cache;
mod config;
mod history;
mod mix;
mod playlist;
mod wrapper;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::index::{Rating, TrackID};

use self::history::History;
pub use self::{
    config::UserDataConfig,
    history::OneListening,
    mix::{Mix, MixID},
    playlist::{Playlist, PlaylistEditAction, PlaylistEntry, PlaylistID},
    wrapper::UserData,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserDataInner {
    config: UserDataConfig,
    history: History,
    track_ratings: HashMap<TrackID, Rating>,
    playlists: HashMap<PlaylistID, Playlist>,
    mixes: HashMap<MixID, Mix>,
}

impl UserDataInner {
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

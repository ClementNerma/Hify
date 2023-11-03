mod cache;
mod config;
mod history;
mod playlist;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::index::{Index, Rating, TrackID};

pub use self::{
    config::UserDataConfig,
    history::OneListening,
    playlist::{Playlist, PlaylistID},
};

use self::{cache::UserDataCache, history::History};

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

pub struct UserDataWrapper {
    inner: UserData,
    cache: UserDataCache,
    on_change: Box<dyn Fn(&UserData) + Send + Sync>,
}

impl UserDataWrapper {
    pub fn new(inner: UserData, on_change: Box<dyn Fn(&UserData) + Send + Sync>) -> Self {
        Self {
            cache: UserDataCache::new(&inner.history, inner.config),
            inner,
            on_change,
        }
    }

    pub fn cache(&self) -> &UserDataCache {
        &self.cache
    }

    pub fn history(&self) -> &History {
        &self.inner.history
    }

    pub fn track_ratings(&self) -> &HashMap<TrackID, Rating> {
        &self.inner.track_ratings
    }

    pub fn playlists(&self) -> &HashMap<PlaylistID, Playlist> {
        &self.inner.playlists
    }

    pub fn set_track_rating(&mut self, track_id: TrackID, rating: Option<Rating>) {
        match rating {
            Some(rating) => {
                self.inner.track_ratings.insert(track_id, rating);
            }

            None => {
                self.inner.track_ratings.remove(&track_id);
            }
        }

        (self.on_change)(&self.inner);
    }

    pub fn log_listening(&mut self, entry: OneListening) {
        if entry.duration_s < self.inner.config.listening_duration_thresold {
            return;
        }

        self.cache.update_with(&entry);
        self.inner.history.push(entry);

        (self.on_change)(&self.inner);
    }

    pub fn create_playlist(&mut self, name: String) -> PlaylistID {
        let playlist = Playlist::new(name);
        let playlist_id = playlist.id;

        self.inner.playlists.insert(playlist.id, playlist);

        (self.on_change)(&self.inner);

        playlist_id
    }

    pub fn add_track_to_playlist(
        &mut self,
        playlist_id: PlaylistID,
        track_id: TrackID,
        position: Option<usize>,
    ) -> Result<(), &'static str> {
        let playlist = self
            .inner
            .playlists
            .get_mut(&playlist_id)
            .ok_or("Playlist was not found")?;

        if matches!(position, Some(position) if position >= playlist.tracks.len()) {
            return Err("Provided position is out-of-bounds");
        }

        match position {
            Some(position) => playlist.tracks.insert(position, track_id),
            None => playlist.tracks.push(track_id),
        }

        (self.on_change)(&self.inner);

        Ok(())
    }

    pub fn cleanup(&mut self, new_index: &Index) {
        self.inner.history.cleanup(new_index);
        self.cache.cleanup(new_index);

        for playlist in self.inner.playlists.values_mut() {
            playlist.cleanup(new_index);
        }

        (self.on_change)(&self.inner);
    }
}

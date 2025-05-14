use std::collections::HashMap;

use crate::index::{Index, Rating, TrackID};

use super::{
    cache::UserDataCache, history::History, playlist::PlaylistEditAction, Mix, MixID, OneListening,
    Playlist, PlaylistID, UserDataInner,
};

pub struct UserData {
    inner: UserDataInner,
    cache: UserDataCache,
    on_change: Box<dyn Fn(&UserDataInner) + Send + Sync>,
}

impl UserData {
    pub fn new(inner: UserDataInner, on_change: Box<dyn Fn(&UserDataInner) + Send + Sync>) -> Self {
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

    pub fn log_listening(&mut self, entry: OneListening) -> Result<(), String> {
        if let Some(last) = self.inner.history.entries().last() {
            if let Some(overlapping_for) = entry.is_overlapping_prev(last) {
                return Err(format!(
                    "Entries overlap in listening history (of about {overlapping_for}):\n* {last:?}\n* {entry:?}",
                ));
            }
        }

        if entry.duration_s < self.inner.config.listening_duration_thresold {
            return Ok(());
        }

        self.cache.update_with(&entry);
        self.inner.history.push(entry);

        (self.on_change)(&self.inner);
        Ok(())
    }

    pub fn create_playlist(&mut self, name: String, tracks: Vec<TrackID>) -> PlaylistID {
        let playlist = Playlist::new(name, tracks);
        let playlist_id = playlist.id;

        self.inner.playlists.insert(playlist.id, playlist);

        (self.on_change)(&self.inner);

        playlist_id
    }

    pub fn edit_playlist(
        &mut self,
        playlist_id: PlaylistID,
        action: PlaylistEditAction,
    ) -> Result<(), &'static str> {
        let playlist = self
            .inner
            .playlists
            .get_mut(&playlist_id)
            .ok_or("Playlist was not found")?;

        playlist.edit(action)?;

        (self.on_change)(&self.inner);

        Ok(())
    }

    pub fn delete_playlist(&mut self, playlist_id: PlaylistID) -> Result<(), &'static str> {
        self.inner
            .playlists
            .remove(&playlist_id)
            .ok_or("Playlist was not found")?;

        (self.on_change)(&self.inner);

        Ok(())
    }

    pub fn register_mix(&mut self, mix: Mix) {
        self.inner.mixes.insert(mix.id(), mix);

        (self.on_change)(&self.inner);
    }

    pub fn get_next_tracks_of_mix<T>(
        &mut self,
        mix_id: MixID,
        max_tracks: usize,
        mapper: impl Fn(TrackID) -> T,
    ) -> Result<Vec<T>, &'static str> {
        let mix = self
            .inner
            .mixes
            .get_mut(&mix_id)
            .ok_or("Mix was not found")?;

        let tracks = mix.next_tracks(max_tracks, mapper);

        (self.on_change)(&self.inner);

        Ok(tracks)
    }

    pub fn cleanup(&mut self, new_index: &Index) {
        self.inner.history.cleanup(new_index);
        self.cache.cleanup(new_index);

        for playlist in self.inner.playlists.values_mut() {
            playlist.cleanup(new_index);
        }

        for mix in self.inner.mixes.values_mut() {
            mix.cleanup(new_index);
        }

        (self.on_change)(&self.inner);
    }
}

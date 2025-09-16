use async_graphql::SimpleObject;

use crate::{index::Index, userdata::UserDataWrapper};

#[derive(SimpleObject)]
pub struct LibraryStats {
    pub tracks_count: usize,
    pub albums_count: usize,
    pub album_artists_count: usize,
    pub artists_count: usize,
    pub total_tracks_listened: usize,
    pub total_listening_time: u64,
}

pub fn generate_stats(index: &Index, user_data: &UserDataWrapper) -> LibraryStats {
    LibraryStats {
        tracks_count: index.tracks.len(),
        album_artists_count: index.album_artists_infos.len(),
        albums_count: index.albums_infos.len(),
        artists_count: index.artists_infos.len(),
        total_tracks_listened: user_data.history().entries().len(),
        total_listening_time: user_data
            .history()
            .entries()
            .iter()
            .map(|entry| u64::from(entry.duration_s))
            .sum(),
    }
}

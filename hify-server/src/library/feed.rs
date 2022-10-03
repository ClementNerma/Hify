use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use async_graphql::{InputObject, SimpleObject};
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    index::{AlbumInfos, ArtistInfos, Index, Track, TrackID},
    userdata::UserDataWrapper,
};

#[derive(SimpleObject)]
pub struct Feed {
    last_listened_to: Vec<Track>,
    popular_tracks: Vec<Track>,
    popular_albums: Vec<AlbumInfos>,
    popular_artists: Vec<ArtistInfos>,
    random_great_albums: Vec<AlbumInfos>,
    random_great_artists: Vec<ArtistInfos>,
}

#[derive(InputObject)]
pub struct FeedParams {
    min_rating: Option<f64>,
    max_items: Option<u8>,
}

pub fn generate_feed(index: &Index, user_data: &UserDataWrapper, params: FeedParams) -> Feed {
    let min_rating = params.min_rating.unwrap_or(80.0);
    let max_items = usize::from(params.max_items.unwrap_or(50));

    let last_listened_to = user_data
        .history()
        .entries()
        .iter()
        .filter_map(|entry| index.tracks.get(&entry.track_id))
        .take(max_items)
        .cloned()
        .collect();

    let popular_tracks: Vec<_> = get_popular_tracks(user_data)
        .filter_map(|id| index.tracks.get(id))
        .take(max_items)
        .cloned()
        .collect();

    let popular_albums = dedup_clone(
        popular_tracks
            .iter()
            .map(|track| track.metadata.tags.get_album_infos())
            .collect(),
    );

    let popular_artists = dedup_clone(
        popular_tracks
            .iter()
            .flat_map(|track| track.metadata.tags.get_album_artists_infos())
            .collect(),
    );

    let random_great_albums = get_random_great(
        &index.cache.albums_mean_score,
        |album_id| index.cache.albums_infos.get(album_id).unwrap().clone(),
        min_rating,
        max_items,
    );

    let random_great_artists = get_random_great(
        &index.cache.albums_artists_mean_score,
        |artist_id| index.cache.artists_infos.get(artist_id).unwrap().clone(),
        min_rating,
        max_items,
    );

    Feed {
        last_listened_to,
        popular_tracks,
        popular_albums,
        popular_artists,
        random_great_albums,
        random_great_artists,
    }
}

fn get_popular_tracks(user_data: &UserDataWrapper) -> impl Iterator<Item = &TrackID> {
    let mut popular_tracks: Vec<_> = user_data.cache().listening_durations().iter().collect();
    popular_tracks.sort_by_key(|(_, count)| u32::MAX - **count);
    popular_tracks.into_iter().map(|(id, _)| id)
}

fn get_random_great<T, U>(
    mean_scores: &HashMap<T, f64>,
    mapper: impl Fn(&T) -> U,
    min_rating: f64,
    max_items: usize,
) -> Vec<U> {
    let mut great: Vec<_> = mean_scores
        .iter()
        .filter(|(_, mean_score)| **mean_score >= min_rating)
        .map(|(id, _)| id)
        .take(max_items)
        .map(mapper)
        .collect();

    great.shuffle(&mut thread_rng());
    great
}

fn dedup_clone<T: Eq + Hash + Clone>(mut v: Vec<T>) -> Vec<T> {
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(e.clone()));
    v
}

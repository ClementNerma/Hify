use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    hash::Hash,
};

use async_graphql::{Enum, InputObject, SimpleObject};
use jiff::{Span, Zoned};
use rand::{rng, seq::SliceRandom};

use crate::{
    index::{AlbumInfos, ArtistInfos, Index, Rating, Track, TrackID},
    userdata::UserData,
};

#[derive(SimpleObject)]
pub struct Feed {
    last_listened_to: Vec<Track>,
    popular_tracks: Vec<Track>,
    popular_albums: Vec<AlbumInfos>,
    popular_artists: Vec<ArtistInfos>,
    periodically_popular_tracks: Vec<Track>,
    periodically_popular_albums: Vec<AlbumInfos>,
    periodically_popular_artists: Vec<ArtistInfos>,
    random_great_albums: Vec<AlbumInfos>,
    random_great_artists: Vec<ArtistInfos>,
    most_recent_albums: Vec<AlbumInfos>,
}

#[derive(InputObject)]
pub struct FeedParams {
    min_rating: Rating,
    max_items: usize,
    popularity_period: Option<PopularityPeriod>,
}

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
pub enum PopularityPeriod {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

pub fn generate_feed(
    index: &Index,
    user_data: &UserData,
    FeedParams {
        min_rating,
        max_items,
        popularity_period,
    }: FeedParams,
) -> Feed {
    let last_listened_to = user_data
        .cache()
        .dedup_history()
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

    let periodically_popular_tracks: Vec<_> = get_periodically_popular_tracks(
        user_data,
        popularity_period.unwrap_or(PopularityPeriod::Weekly),
    )
    .filter_map(|id| index.tracks.get(id))
    .take(max_items)
    .cloned()
    .collect();

    let periodically_popular_albums = dedup_clone(
        periodically_popular_tracks
            .iter()
            .map(|track| track.metadata.tags.get_album_infos())
            .collect(),
    );

    let periodically_popular_artists = dedup_clone(
        periodically_popular_tracks
            .iter()
            .flat_map(|track| track.metadata.tags.get_album_artists_infos())
            .collect(),
    );

    let random_great_albums = get_random_great(
        &index.cache.albums_mean_score,
        |album_id| index.cache.albums_infos.get(&album_id).unwrap().clone(),
        min_rating,
        max_items,
    );

    let random_great_artists = get_random_great(
        &index.cache.album_artists_mean_score,
        |artist_id| index.cache.artists_infos.get(&artist_id).unwrap().clone(),
        min_rating,
        max_items,
    );

    let most_recent_albums = index
        .cache
        .most_recent_albums
        .iter()
        .take(max_items)
        .map(|album_id| index.cache.albums_infos.get(album_id).unwrap().clone())
        .collect::<Vec<_>>();

    Feed {
        last_listened_to,
        popular_tracks,
        popular_albums,
        popular_artists,
        periodically_popular_tracks,
        periodically_popular_albums,
        periodically_popular_artists,
        random_great_albums,
        random_great_artists,
        most_recent_albums,
    }
}

fn get_popular_tracks(user_data: &UserData) -> impl Iterator<Item = &TrackID> {
    let mut popular_tracks: Vec<_> = user_data.cache().listening_durations().iter().collect();
    popular_tracks.sort_by_key(|(_, count)| u32::MAX - **count);
    popular_tracks.into_iter().map(|(id, _)| id)
}

fn get_periodically_popular_tracks(
    user_data: &UserData,
    period: PopularityPeriod,
) -> impl Iterator<Item = &TrackID> {
    let mut popular_tracks = HashMap::new();

    let period = match period {
        PopularityPeriod::Daily => Span::new(),
        PopularityPeriod::Weekly => Span::new().weeks(1),
        PopularityPeriod::Monthly => Span::new().months(1),
        PopularityPeriod::Yearly => Span::new().years(1),
    };

    let start_period = Zoned::now().checked_sub(period).unwrap().timestamp();

    let listenings = user_data
        .history()
        .entries()
        .iter()
        .rev()
        .filter(|pred| pred.ended_at >= start_period);

    for listening in listenings {
        match popular_tracks.entry(&listening.track_id) {
            Entry::Occupied(mut entry) => *entry.get_mut() += listening.duration_s,
            Entry::Vacant(entry) => {
                entry.insert(listening.duration_s);
            }
        }
    }

    let mut popular_tracks: Vec<_> = popular_tracks.into_iter().collect();

    popular_tracks.sort_by_key(|(_, count)| u32::MAX - *count);
    popular_tracks.into_iter().map(|(id, _)| id)
}

fn get_random_great<T: Copy + Eq + Hash, U>(
    mean_scores: &HashMap<T, f64>,
    mapper: impl Fn(T) -> U,
    min_rating: Rating,
    max_items: usize,
) -> Vec<U> {
    let mut out = HashSet::with_capacity(max_items);
    let min_rating = f64::from(min_rating.value());

    for (item_id, mean_score) in mean_scores.iter() {
        if *mean_score >= min_rating {
            out.insert(*item_id);

            if out.len() >= max_items {
                break;
            }
        }
    }

    let mut great = out.into_iter().map(mapper).collect::<Vec<_>>();

    great.shuffle(&mut rng());
    great
}

fn dedup_clone<T: Eq + Hash + Clone>(mut v: Vec<T>) -> Vec<T> {
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(e.clone()));
    v
}

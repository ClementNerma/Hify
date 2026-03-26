use serde::Deserialize;

use crate::{
    index::{ArtistID, GenreID, IndexCache, Rating},
    manager::Ratings,
    utils::{Rng, deterministic_shuffle},
};

use super::{
    dtos::TrackCompleteInfos,
    pagination::{Paginated, Pagination},
};

pub fn generate_mix(
    params: UserMixParams,
    index: &IndexCache,
    ratings: &Ratings,
    pagination: Pagination,
) -> Paginated<TrackCompleteInfos> {
    let UserMixParams {
        source,
        filter,
        seed,
    } = params;

    #[allow(
        clippy::needless_collect,
        reason = "required to get an ExactSizeIterator"
    )]
    let mut tracks = index
        .tracks
        .values()
        .filter(|track| match source {
            UserMixSource::All => true,
            UserMixSource::Artist(artist_id) => track.tags.artists_id.contains(&artist_id),
            UserMixSource::Genre(genre_id) => track.tags.genres_id.contains(&genre_id),
        })
        .filter(|track| match filter {
            UserMixFilter::NotRated => ratings.get(&track.id).is_none(),

            UserMixFilter::NotBadlyRated => ratings
                .get(&track.id)
                .is_none_or(|rating| *rating > Rating::Two),

            UserMixFilter::WellRated => ratings
                .get(&track.id)
                .is_some_and(|rating| *rating >= Rating::Four),

            UserMixFilter::BestRated => ratings
                .get(&track.id)
                .is_some_and(|rating| *rating == Rating::Five),

            UserMixFilter::IncludeAll => true,
        })
        .collect::<Vec<_>>();

    deterministic_shuffle(
        &mut tracks,
        &mut Rng::with_seed(seed, Rng::DEFAULT_INCREMENT),
    );

    Paginated::paginate(
        tracks
            .into_iter()
            .map(|track| TrackCompleteInfos::new(track.clone(), index, ratings)),
        pagination,
    )
}

#[derive(Deserialize, Clone, Copy)]
pub struct UserMixParams {
    source: UserMixSource,
    filter: UserMixFilter,
    seed: u64,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase", tag = "type", content = "id")]
pub enum UserMixSource {
    All,
    Artist(ArtistID),
    Genre(GenreID),
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserMixFilter {
    NotRated,      // rating = None
    NotBadlyRated, // rating = Some(>= 3)
    WellRated,     // rating = Some(>= 4)
    BestRated,     // rating = Some(5)
    IncludeAll,    // rating = *
}

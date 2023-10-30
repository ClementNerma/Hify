use std::collections::HashSet;

use async_graphql::InputObject;
use rand::{seq::SliceRandom, thread_rng};

use crate::index::{ArtistID, GenreID, Index, Rating, Track, TrackID};

#[derive(InputObject)]
pub struct MixParams {
    min_rating: Option<Rating>,
    from_artists: Option<HashSet<ArtistID>>,
    from_genres: Option<HashSet<GenreID>>,
    exclude_tracks: Option<HashSet<TrackID>>,
}

pub fn generate_mix(index: &Index, params: MixParams, max_tracks: usize) -> Vec<Track> {
    #[deny(unused_variables)]
    let MixParams {
        min_rating,
        from_artists,
        from_genres,
        exclude_tracks,
    } = params;

    let mut tracks: Vec<_> = index
        .tracks
        .values()
        .filter(|track| match exclude_tracks {
            Some(ref exclude_tracks) => exclude_tracks.contains(&track.id),
            None => true,
        })
        .filter(|track| match min_rating {
            Some(min_rating) => track.metadata.tags.rating.unwrap_or(Rating::Zero) >= min_rating,
            None => true,
        })
        .filter(|track| match from_artists {
            Some(ref artist_ids) => index
                .cache
                .tracks_all_artists
                .get(&track.id)
                .unwrap()
                .intersection(artist_ids)
                .next()
                .is_some(),
            None => true,
        })
        .filter(|track| match &from_genres {
            Some(ref genre_ids) => track
                .metadata
                .tags
                .get_genres_infos()
                .any(|genre| genre_ids.contains(&genre.get_id())),
            None => true,
        })
        .collect();

    tracks.shuffle(&mut thread_rng());

    tracks.into_iter().take(max_tracks).cloned().collect()
}

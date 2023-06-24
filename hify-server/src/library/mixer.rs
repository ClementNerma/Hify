use async_graphql::InputObject;
use rand::{seq::SliceRandom, thread_rng};

use crate::index::{ArtistID, GenreID, Index, Rating, Track};

#[derive(InputObject)]
pub struct MixParams {
    min_rating: Option<Rating>,
    max_tracks: Option<u8>,
    from_artist: Option<ArtistID>,
    from_genre: Option<GenreID>,
}

pub fn generate_mix(index: &Index, params: MixParams) -> Vec<Track> {
    #[deny(unused_variables)]
    let MixParams {
        min_rating,
        max_tracks,
        from_artist,
        from_genre,
    } = params;

    let min_rating = min_rating.unwrap_or(Rating::Eight);
    let max_tracks = usize::from(max_tracks.unwrap_or(200));

    let mut tracks: Vec<_> = index
        .tracks
        .values()
        .filter(|track| match track.metadata.tags.rating {
            None => false,
            Some(rating) => rating >= min_rating,
        })
        .filter(|track| match &from_artist {
            Some(artist_id) => index
                .cache
                .tracks_all_artists
                .get(&track.id)
                .unwrap()
                .contains(artist_id),
            None => true,
        })
        .filter(|track| match &from_genre {
            Some(genre_id) => track
                .metadata
                .tags
                .get_genres_infos()
                .any(|genre| &genre.get_id() == genre_id),
            None => true,
        })
        .collect();

    tracks.shuffle(&mut thread_rng());

    tracks.into_iter().take(max_tracks).cloned().collect()
}

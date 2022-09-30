use async_graphql::InputObject;
use rand::{seq::SliceRandom, thread_rng};

use crate::index::{ArtistID, ArtistInfos, GenreID, Index, Track};

#[derive(InputObject)]
pub struct MixParams {
    min_rating: Option<u8>,
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

    let min_rating = min_rating.unwrap_or(80);
    let max_tracks = usize::from(max_tracks.unwrap_or(200));

    let mut tracks: Vec<_> = index
        .tracks
        .values()
        .filter(|track| match track.metadata.tags.rating {
            None => min_rating == 0,
            Some(rating) => rating >= min_rating,
        })
        // TODO: awfully unoptimized
        .filter(|track| match &from_artist {
            Some(artist_id) => {
                let album_artists = &index
                    .cache
                    .albums_infos
                    .get(&track.metadata.tags.get_album_infos().get_id())
                    .unwrap()
                    .album_artists;

                let mut all_artists_iter = album_artists.iter().map(ArtistInfos::get_id).chain(
                    track
                        .metadata
                        .tags
                        .get_artists_infos()
                        .map(|infos| infos.get_id()),
                );

                all_artists_iter.any(|id| &id == artist_id)
            }
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

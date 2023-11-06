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

    let mut tracks = index
        .tracks
        .values()
        .filter(|track| {
            if let Some(ref exclude_tracks) = exclude_tracks {
                if exclude_tracks.contains(&track.id) {
                    return false;
                }
            }

            if let Some(min_rating) = min_rating {
                if track.metadata.tags.rating.unwrap_or(Rating::Zero) < min_rating {
                    return false;
                }
            }

            if let Some(ref artist_ids) = from_artists {
                if index
                    .cache
                    .tracks_all_artists
                    .get(&track.id)
                    .unwrap()
                    .intersection(artist_ids)
                    .next()
                    .is_none()
                {
                    return false;
                }
            }

            if let Some(ref genre_ids) = from_genres {
                if !track
                    .metadata
                    .tags
                    .get_genres_infos()
                    .any(|genre| genre_ids.contains(&genre.get_id()))
                {
                    return false;
                }
            }

            true
        })
        .collect::<Vec<_>>();

    tracks.shuffle(&mut thread_rng());

    tracks.into_iter().take(max_tracks).cloned().collect()
}

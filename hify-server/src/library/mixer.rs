use std::collections::HashSet;

use async_graphql::{Enum, InputObject};
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    index::{ArtistID, GenreID, Index, Rating, Track, TrackID},
    userdata::UserDataWrapper,
};

#[derive(InputObject)]
pub struct MixParams {
    source: MixSource,
    ordering: MixOrdering,
    min_rating: Option<Rating>,
    from_artists: Option<HashSet<ArtistID>>,
    from_genres: Option<HashSet<GenreID>>,
    exclude_tracks: Option<HashSet<TrackID>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Enum)]
pub enum MixSource {
    AllTracks,
    History,
}

#[derive(Clone, Copy, PartialEq, Eq, Enum)]
pub enum MixOrdering {
    Random,
    ListeningDuration,
    LastListening,
}

pub fn generate_mix(
    index: &Index,
    user_data: &UserDataWrapper,
    params: MixParams,
    max_tracks: usize,
) -> Vec<Track> {
    let MixParams {
        source,
        ordering,
        min_rating,
        from_artists,
        from_genres,
        exclude_tracks,
    } = params;

    let tracks: Box<dyn Iterator<Item = TrackID>> = match source {
        MixSource::AllTracks => Box::new(index.tracks.values().map(|track| track.id)),
        MixSource::History => Box::new(
            user_data
                .cache()
                .dedup_history()
                .iter()
                .map(|entry| entry.track_id),
        ),
    };

    let mut tracks = tracks
        .filter_map(|track_id| {
            let track = index.tracks.get(&track_id).unwrap();

            if let Some(ref exclude_tracks) = exclude_tracks {
                if exclude_tracks.contains(&track.id) {
                    return None;
                }
            }

            if let Some(min_rating) = min_rating {
                if track.metadata.tags.rating.unwrap_or(Rating::Zero) < min_rating {
                    return None;
                }
            }

            if let Some(ref artist_ids) = from_artists {
                index
                    .cache
                    .tracks_all_artists
                    .get(&track.id)
                    .unwrap()
                    .intersection(artist_ids)
                    .next()?;
            }

            if let Some(ref genre_ids) = from_genres {
                if !track
                    .metadata
                    .tags
                    .get_genres_infos()
                    .any(|genre| genre_ids.contains(&genre.get_id()))
                {
                    return None;
                }
            }

            Some(track.clone())
        })
        .collect::<Vec<_>>();

    match ordering {
        MixOrdering::Random => tracks.shuffle(&mut thread_rng()),

        MixOrdering::ListeningDuration => {
            tracks.sort_by_key(|track| {
                user_data
                    .cache()
                    .listening_durations()
                    .get(&track.id)
                    .unwrap_or(&0)
            });

            tracks.reverse();
        }

        MixOrdering::LastListening => {
            tracks.retain(|track| user_data.cache().last_listening().get(&track.id).is_some());
            tracks.sort_by_key(|track| user_data.cache().last_listening().get(&track.id));
        }
    }

    tracks.into_iter().take(max_tracks).collect()
}

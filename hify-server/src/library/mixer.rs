use std::collections::HashSet;

use async_graphql::{Enum, InputObject, OneofObject};
use rand::{rng, seq::SliceRandom};

use crate::{
    graphql::EmptyScalar,
    index::{ArtistID, GenreID, Index, Rating},
    userdata::{Mix, PlaylistID, UserData},
};

pub fn generate_mix(
    index: &Index,
    user_data: &UserData,
    params: MixParams,
) -> Result<Mix, &'static str> {
    let MixParams {
        source,
        ordering,
        min_rating,
        from_artists,
        from_genres,
    } = &params;

    let source_tracks: Vec<_> = match source {
        MixSource::AllTracks(EmptyScalar) => index.tracks.values().map(|track| track.id).collect(),

        MixSource::History(EmptyScalar) => user_data
            .cache()
            .dedup_history()
            .iter()
            .map(|entry| entry.track_id)
            .collect(),

        MixSource::Playlist(PlaylistSourceParams { playlist_id }) => {
            let playlist = user_data
                .playlists()
                .get(playlist_id)
                .ok_or("Provided playlist ID was not found")?;

            playlist
                .entries
                .iter()
                .map(|entry| entry.track_id)
                .collect()
        }
    };

    let mut tracks = source_tracks
        .into_iter()
        .filter(|track_id| {
            let track = index.tracks.get(track_id).unwrap();

            if let Some(min_rating) = min_rating {
                if track.metadata.tags.rating.unwrap_or(Rating::Zero) < *min_rating {
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

    match ordering {
        MixOrdering::Random => tracks.shuffle(&mut rng()),

        MixOrdering::ListeningDuration => {
            tracks.sort_by_key(|track_id| {
                user_data
                    .cache()
                    .listening_durations()
                    .get(track_id)
                    .unwrap_or(&0)
            });

            tracks.reverse();
        }

        MixOrdering::LastListening => {
            tracks.sort_by_key(|track_id| user_data.cache().last_listening().get(track_id));
        }
    }

    Ok(Mix::new(tracks))
}

#[derive(InputObject)]
pub struct MixParams {
    source: MixSource,
    ordering: MixOrdering,
    min_rating: Option<Rating>,
    from_artists: Option<HashSet<ArtistID>>,
    from_genres: Option<HashSet<GenreID>>,
}

#[derive(Clone, OneofObject)]
pub enum MixSource {
    AllTracks(EmptyScalar),
    History(EmptyScalar),
    Playlist(PlaylistSourceParams),
}

#[derive(Clone, InputObject)]
pub struct PlaylistSourceParams {
    playlist_id: PlaylistID,
}

#[derive(Clone, Copy, PartialEq, Eq, Enum)]
pub enum MixOrdering {
    Random,
    ListeningDuration,
    LastListening,
}

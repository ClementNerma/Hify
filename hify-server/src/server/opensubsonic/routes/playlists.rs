use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use serde::Deserialize;

use crate::{
    index::{IndexCache, Rating, TrackID},
    os_struct,
    server::{
        HttpState,
        opensubsonic::{
            OSCommonParams, OSNestedResponse, OSResult,
            convert::{to_iso_8601, track_to_child},
            types::PlaylistWithSongs,
        },
    },
    stable_hash,
};

use super::OpenSubsonicRouter;

pub fn router() -> OpenSubsonicRouter {
    OpenSubsonicRouter::new()
        .route("/getPlaylists", get_playlists)
        .route("/getPlaylist", get_playlist)
}

os_struct!(pub struct GetPlaylistsAnswer { #[children] { playlist: Vec<PlaylistWithSongs> } });

async fn get_playlists(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    State(state): State<HttpState>,
    // TODO: query parameters
) -> OSNestedResponse<GetPlaylistsAnswer> {
    let index = state.index().await;
    let ratings = state.ratings().await;

    OSNestedResponse(
        f,
        "playlists",
        GetPlaylistsAnswer {
            // TODO: this returns a 'empty' playlist for each artist and genre, whereas the server does not need the tracks.
            playlist: auto_playlists(&index, &ratings),
        },
    )
}

#[derive(Deserialize)]
struct GetPlaylistParams {
    id: String,
}

async fn get_playlist(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    Query(GetPlaylistParams { id }): Query<GetPlaylistParams>,
    State(state): State<HttpState>,
) -> OSResult<PlaylistWithSongs> {
    let index = state.index().await;
    let ratings = state.ratings().await;

    auto_playlists(&index, &ratings)
        .into_iter()
        .find(|playlist| playlist.id == id)
        .ok_or((StatusCode::NOT_FOUND, "Provided playlist ID was not found"))?;

    Ok(OSNestedResponse(
        f,
        "playlist",
        auto_playlists(&index, &ratings)
            .into_iter()
            .find(|playlist| playlist.id == id)
            .unwrap(),
    ))
}

// TODO: make it configurable (opt-in or opt-out)
#[allow(clippy::too_many_lines)]
fn auto_playlists(
    index: &IndexCache,
    ratings: &HashMap<TrackID, Rating>,
) -> Vec<PlaylistWithSongs> {
    let mut playlists: Vec<(String, Vec<TrackID>)> = vec![];

    // Add score-based playlists
    playlists.push((
        "Top Rated".to_string(),
        index
            .tracks
            .values()
            .filter(|track| {
                ratings
                    .get(&track.id)
                    .is_some_and(|rating| *rating == Rating::Five)
            })
            .map(|track| track.id)
            .collect(),
    ));

    playlists.push((
        "Great Tracks".to_string(),
        index
            .tracks
            .values()
            .filter(|track| {
                ratings
                    .get(&track.id)
                    .is_some_and(|rating| *rating >= Rating::Four)
            })
            .map(|track| track.id)
            .collect(),
    ));

    // Add artist-based playlists
    let mut artists = index.artists.values().collect::<Vec<_>>();
    artists.sort_by(|a, b| a.name.cmp(&b.name));

    for artist in artists {
        let track_ids = index.artists_tracks.get(&artist.id).unwrap();

        playlists.push((
            format!("Best Of Artist: {}", artist.name),
            track_ids
                .iter()
                .filter(|track_id| {
                    ratings
                        .get(track_id)
                        .is_some_and(|rating| *rating >= Rating::Four)
                })
                .copied()
                .collect(),
        ));
    }

    // Add genre-based playlists
    let mut genres = index.genres.values().collect::<Vec<_>>();
    genres.sort_by(|a, b| a.name.cmp(&b.name));

    for genre in genres {
        let track_ids = index.genres_tracks.get(&genre.id).unwrap();

        playlists.push((
            format!("Best Of Genre: {}", genre.name),
            track_ids
                .iter()
                .filter(|track_id| {
                    ratings
                        .get(track_id)
                        .is_some_and(|rating| *rating >= Rating::Four)
                })
                .copied()
                .collect(),
        ));
    }

    // Convert playlists to OpenSubsonic format
    playlists
        .into_iter()
        // Remove empty playlists (cannot get created / changed dates from them)
        .filter(|(_, track_ids)| !track_ids.is_empty())
        .map(|(name, track_ids)| {
            let tracks: Vec<_> = track_ids
                .into_iter()
                .map(|track_id| index.tracks.get(&track_id).unwrap())
                .collect();

            PlaylistWithSongs {
                id: format!("{:X}", stable_hash!(name)),
                name,
                comment: Some("Auto-generated".to_owned()),
                owner: None,
                public: Some(false),
                song_count: tracks.len(),
                duration_s: tracks.iter().map(|track| track.metadata.duration_s).sum(),
                created_iso_8601: to_iso_8601(
                    tracks
                        .iter()
                        .map(|track| track.file_times.mtime)
                        .min()
                        .unwrap(),
                ),
                changed_iso_8601: to_iso_8601(
                    tracks
                        .iter()
                        .map(|track| track.file_times.mtime)
                        .max()
                        .unwrap(),
                ),
                cover_art_id: None,
                readonly: Some(true),
                valid_until_iso_8601: None,
                tracks: tracks
                    .into_iter()
                    .map(|track| track_to_child(track, index, ratings))
                    .collect(),
            }
        })
        .collect()
}

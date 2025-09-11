use std::sync::Arc;

use axum::{extract::Query, Extension};
use serde::Deserialize;

use crate::{
    http::{
        opensubsonic::{
            convert::{album_to_id3_with_songs, artist_to_id3, track_to_child},
            types::{AlbumID3WithSongs, ArtistID3, Child},
            OSCommonParams, OSNestedResponse,
        },
        HttpState,
    },
    index::{search_index, IndexSearchResults, SearchOptions},
    os_struct,
};

use super::OpenSubsonicRouter;

pub fn router() -> OpenSubsonicRouter {
    OpenSubsonicRouter::new().route("/search3", search3)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Search3Params {
    query: String,
    artist_count: Option<usize>,
    artist_offset: Option<usize>,
    album_count: Option<usize>,
    album_offset: Option<usize>,
    song_count: Option<usize>,
    song_offset: Option<usize>,
    music_folder_id: Option<u64>,
}

os_struct! {
    pub struct Search3Answer {
        #[children] {
            artist: Vec<ArtistID3>,
            album: Vec<AlbumID3WithSongs>,// TODO: Technically should be without songs
            song: Vec<Child>
        }
    }
}

async fn search3(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    Query(params): Query<Search3Params>,
    Extension(state): Extension<Arc<HttpState>>,
) -> OSNestedResponse<Search3Answer> {
    let Search3Params {
        query,
        artist_count,
        artist_offset,
        album_count,
        album_offset,
        song_count,
        song_offset,
        music_folder_id,
    } = params;

    let index = state.index.read().await;
    let user_data = state.user_data.read().await;

    // Special search cases
    let results = if query.is_empty() || query == "\"\"" {
        IndexSearchResults {
            tracks: index
                .tracks
                .iter()
                .skip(song_offset.unwrap_or(0))
                .take(song_count.unwrap_or(usize::MAX))
                .map(|(_, track)| track.clone())
                .collect(),

            albums: index
                .cache
                .albums_infos
                .values()
                .skip(album_offset.unwrap_or(0))
                .take(album_count.unwrap_or(usize::MAX))
                .cloned()
                .collect(),

            artists: index
                .cache
                .artists_infos
                .values()
                .skip(artist_offset.unwrap_or(0))
                .take(artist_count.unwrap_or(usize::MAX))
                .cloned()
                .collect(),
        }
    } else {
        search_index(
            &index,
            // TODO
            SearchOptions {
                search_cache: None,
                tracks_user_score: None,
            },
            &query,
            usize::MAX, // TODO (keep it MAX when query is empty though)
        )
    };

    let IndexSearchResults {
        tracks,
        albums,
        artists,
    } = results;

    OSNestedResponse(
        f,
        "searchResult3",
        Search3Answer {
            artist: artists
                .iter()
                .map(|artist| artist_to_id3(artist, &index))
                .collect(),

            album: albums
                .iter()
                .map(|album| album_to_id3_with_songs(album, &index, None))
                .collect(),

            song: tracks
                .iter()
                .map(|track| track_to_child(track, &index, &user_data))
                .collect(),
        },
    )
}

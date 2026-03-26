use axum::extract::{Query, State};
use indexmap::IndexMap;
use serde::Deserialize;

use crate::{
    os_struct,
    server::{
        HttpState,
        opensubsonic::{
            OSCommonParams, OSNestedResponse,
            convert::{album_to_id3_with_songs, artist_to_id3, track_to_child},
            types::{AlbumID3WithSongs, ArtistID3, Child, MUSIC_FOLDER_ID},
        },
        utils::{
            pagination::{Paginated, Pagination, PaginationDir},
            search::{search_albums, search_artists, search_tracks},
        },
    },
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
    State(state): State<HttpState>,
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

    if music_folder_id.is_some_and(|id| id != MUSIC_FOLDER_ID) {
        return OSNestedResponse(
            f,
            "searchResult3",
            Search3Answer {
                artist: vec![],
                album: vec![],
                song: vec![],
            },
        );
    }

    let index = state.index().await;
    let ratings = state.ratings().await;

    let tracks = search(
        &query,
        |query, pagination| search_tracks(query, pagination, &index, &ratings),
        &index.tracks,
        Pagination {
            offset: song_offset,
            limit: song_count.unwrap_or(50),
            dir: PaginationDir::Asc,
        },
    );

    let albums = search(
        &query,
        |query, pagination| search_albums(query, pagination, &index),
        &index.albums,
        Pagination {
            offset: album_offset,
            limit: album_count.unwrap_or(50),
            dir: PaginationDir::Asc,
        },
    );

    let artists = search(
        &query,
        |query, pagination| search_artists(query, pagination, &index),
        &index.artists,
        Pagination {
            offset: artist_offset,
            limit: artist_count.unwrap_or(50),
            dir: PaginationDir::Asc,
        },
    );

    OSNestedResponse(
        f,
        "searchResult3",
        Search3Answer {
            artist: artists
                .results
                .iter()
                .map(|artist| artist_to_id3(artist, &index))
                .collect(),

            album: albums
                .results
                .iter()
                .map(|album| album_to_id3_with_songs(album, &index, &ratings))
                .collect(),

            song: tracks
                .results
                .iter()
                .map(|track| track_to_child(track, &index, &ratings))
                .collect(),
        },
    )
}

// TODO: return a Vec<_> directly
fn search<T: Clone, K>(
    query: &str,
    search: impl Fn(&str, Pagination) -> Paginated<T>,
    alt_source: &IndexMap<K, T>,
    pagination: Pagination,
) -> Paginated<T> {
    if query.is_empty() || query == "\"\"" {
        Paginated::paginate(alt_source.values(), pagination).map(T::clone)
    } else {
        search(query, pagination)
    }
}

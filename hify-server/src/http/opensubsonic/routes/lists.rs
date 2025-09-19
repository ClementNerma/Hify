use std::sync::Arc;

use axum::{Extension, extract::Query};
use serde::Deserialize;

use crate::{
    http::{
        HttpState,
        opensubsonic::{
            convert::{album_to_child, album_to_id3_with_songs, track_to_child},
            types::{AlbumID3WithSongs, Artist, Child},
        },
    },
    index::{AlbumInfos, GenreID, GenreInfos, Index},
    os_struct,
};

use super::{
    super::{OSCommonParams, OSNestedResponse},
    OpenSubsonicRouter,
};

pub fn router() -> OpenSubsonicRouter {
    OpenSubsonicRouter::new()
        .route("/getAlbumList", get_album_list)
        .route("/getAlbumList2", get_album_list2)
        .route("/getStarred2", get_starred2)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AlbumListParams {
    #[serde(rename = "type")]
    sort: AlbumListSort,
    size: Option<usize>,
    offset: Option<usize>,
    from_year: Option<u16>,
    to_year: Option<u16>,
    genre: Option<String>,
    // music_folder_id
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum AlbumListSort {
    Random,
    Newest,
    Highest,
    Frequent,
    Recent,
    AlphabeticalByName,
    AlphabeticalByArtist,
    Starred,
    ByYear,
    ByGenre,
}

fn _album_list(params: AlbumListParams, index: &Index) -> impl Iterator<Item = &AlbumInfos> {
    let AlbumListParams {
        sort,
        size,
        offset,
        #[allow(unused_variables)] // TODO
        from_year,
        #[allow(unused_variables)] // TODO
        to_year,
        genre,
    } = params;

    // TODO: take sort into consideration
    // TODO: take from_year + to_year into consideration

    let genre = genre.map(|genre| GenreInfos::new(genre).get_id());

    index
        .albums_infos
        .iter()
        .filter(move |(album_id, _)| {
            genre.is_none_or(|genre| index.albums_genres.get(album_id).unwrap().contains(&genre))
        })
        .skip(offset.unwrap_or(0))
        .take(size.unwrap_or(50))
        .map(|(_, album)| album)
}

os_struct!(pub struct GetAlbumListAnswer {
    #[children] {
        #[rename = "album"]
        albums: Vec<Child>
    }
});

async fn get_album_list(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    Query(params): Query<AlbumListParams>,
    Extension(state): Extension<Arc<HttpState>>,
) -> OSNestedResponse<GetAlbumListAnswer> {
    let index = state.index.read().await;

    OSNestedResponse(
        f,
        "albumList",
        GetAlbumListAnswer {
            albums: _album_list(params, &index)
                .map(|album| album_to_child(album, &index))
                .collect(),
        },
    )
}

os_struct!(pub struct GetAlbumList2Answer {
    #[children] {
        #[rename = "album"]
        // Technically should be `AlbumID3` but eh...
        albums: Vec<AlbumID3WithSongs>
    }
});

async fn get_album_list2(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    Query(params): Query<AlbumListParams>,
    Extension(state): Extension<Arc<HttpState>>,
) -> OSNestedResponse<GetAlbumList2Answer> {
    let index = state.index.read().await;

    OSNestedResponse(
        f,
        "albumList2",
        GetAlbumList2Answer {
            albums: _album_list(params, &index)
                .map(|album| album_to_id3_with_songs(album, &index, None))
                .collect(),
        },
    )
}

os_struct!(pub struct GetStarred2Answer {
    #[children] {
        artist: Vec<Artist>,
        album: Vec<Child>,
        song: Vec<Child>
    }
});

async fn get_starred2(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    Extension(state): Extension<Arc<HttpState>>,
    // TODO: query
) -> OSNestedResponse<GetStarred2Answer> {
    let index = state.index.read().await;
    let user_data = state.user_data.read().await;

    OSNestedResponse(
        f,
        "starred2",
        GetStarred2Answer {
            // TODO: should return explicitly-starred artists, not artists who have starred songs
            artist: vec![],

            // TODO: should return explicitly-starred artists, not artists who have starred songs
            album: vec![],

            song: user_data
                .track_ratings()
                .keys()
                .map(|track| track_to_child(index.tracks.get(track).unwrap(), &index, &user_data))
                .collect(),
        },
    )
}

use std::collections::HashMap;

use axum::extract::{Query, State};
use serde::Deserialize;

use crate::{
    index::{Album, Genre, IndexCache},
    os_struct,
    server::{
        HttpState,
        opensubsonic::{
            convert::{album_to_child, album_to_id3_with_songs, track_to_child},
            types::{AlbumID3WithSongs, Artist, Child},
        },
    },
    utils::Rng,
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

fn album_list(params: AlbumListParams, index: &IndexCache) -> impl Iterator<Item = &Album> {
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

    let genre = genre.map(|genre| Genre::new(genre).id);

    let mut albums = index
        .albums
        .iter()
        .filter(move |(album_id, _)| {
            genre.is_none_or(|genre| index.albums_genres.get(album_id).unwrap().contains(&genre))
        })
        .collect::<Vec<_>>();

    // TODO: Remove this allow
    #[allow(clippy::match_same_arms)]
    match sort {
        AlbumListSort::Random => {
            let mut rng = Rng::new();
            albums.sort_by_key(|_| rng.next_u64());
        }

        AlbumListSort::Newest => {
            let album_recent_pos = index
                .latest_added_albums
                .iter()
                .enumerate()
                .map(|(pos, album_id)| (*album_id, pos))
                .collect::<HashMap<_, _>>();

            albums.sort_by_key(|(album_id, _)| album_recent_pos.get(album_id).unwrap());
        }

        AlbumListSort::Highest => {
            // TODO
        }

        AlbumListSort::Frequent => {
            // TODO
        }

        AlbumListSort::Recent => {
            // TODO
        }

        AlbumListSort::AlphabeticalByName => {
            albums.sort_by_key(|(_, album)| &album.name);
        }

        AlbumListSort::AlphabeticalByArtist => {
            // TODO
        }

        AlbumListSort::Starred => {
            // TODO
        }

        AlbumListSort::ByYear => {
            // TODO
        }

        AlbumListSort::ByGenre => {
            // TODO
        }
    }

    albums
        .into_iter()
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
    State(state): State<HttpState>,
) -> OSNestedResponse<GetAlbumListAnswer> {
    let index = state.index().await;

    OSNestedResponse(
        f,
        "albumList",
        GetAlbumListAnswer {
            albums: album_list(params, &index)
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
    State(state): State<HttpState>,
) -> OSNestedResponse<GetAlbumList2Answer> {
    let index = state.index().await;
    let ratings = state.ratings().await;

    OSNestedResponse(
        f,
        "albumList2",
        GetAlbumList2Answer {
            albums: album_list(params, &index)
                .map(|album| album_to_id3_with_songs(album, &index, &ratings))
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
    State(state): State<HttpState>,
    // TODO: query
) -> OSNestedResponse<GetStarred2Answer> {
    let index = state.index().await;
    let ratings = state.ratings().await;

    OSNestedResponse(
        f,
        "starred2",
        GetStarred2Answer {
            // TODO: should return explicitly-starred artists, not artists who have starred songs
            artist: vec![],

            // TODO: should return explicitly-starred artists, not artists who have starred songs
            album: vec![],

            song: ratings
                .keys()
                .map(|track| track_to_child(index.tracks.get(track).unwrap(), &index, &ratings))
                .collect(),
        },
    )
}

use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use serde::Deserialize;

use crate::{
    arts::ArtSize,
    index::{AlbumID, ArtistID, TrackID},
    os_struct,
    server::{
        HttpState,
        opensubsonic::{
            convert::{album_to_id3_with_songs, track_to_child},
            types::{
                AlbumInfo, ArtistInfo2, Child, CoverArtId, Genre, MUSIC_FOLDER_ID, MusicFolder,
            },
        },
    },
};

use super::{
    super::{OSCommonParams, OSNestedResponse, OSResult, types::AlbumID3WithSongs},
    OpenSubsonicRouter,
    media::make_cover_art_uri,
};

pub fn router() -> OpenSubsonicRouter {
    OpenSubsonicRouter::new()
        .route("/getMusicFolders", get_music_folders)
        .route("/getGenres", get_genres)
        .route("/getAlbum", get_album)
        .route("/getSong", get_song)
        .route("/getArtistInfo2", get_artist_info2)
        .route("/getAlbumInfo2", get_album_info2)
}

os_struct!(pub struct GetMusicFoldersAnswer { #[children] { music_folder: Vec<MusicFolder> } });

async fn get_music_folders(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
) -> OSNestedResponse<GetMusicFoldersAnswer> {
    OSNestedResponse(
        f,
        "musicFolders",
        GetMusicFoldersAnswer {
            music_folder: vec![MusicFolder {
                id: MUSIC_FOLDER_ID,
                name: "Music Library",
            }],
        },
    )
}

os_struct!(pub struct GetGenresAnswer { #[children] { genre: Vec<Genre> } });

async fn get_genres(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    State(state): State<HttpState>,
) -> OSNestedResponse<GetGenresAnswer> {
    let index = state.index().await;

    OSNestedResponse(
        f,
        "genres",
        GetGenresAnswer {
            genre: index
                .genres
                .iter()
                .map(|(id, genre)| {
                    let tracks = index.genres_tracks.get(id).unwrap();
                    let albums = index.genres_albums.get(id).unwrap();

                    Genre {
                        name: genre.name.clone(),
                        song_count: tracks.len(),
                        album_count: albums.len(),
                    }
                })
                .collect(),
        },
    )
}

#[derive(Deserialize)]
struct GetAlbumParams {
    id: AlbumID,
}

async fn get_album(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    Query(GetAlbumParams { id }): Query<GetAlbumParams>,
    State(state): State<HttpState>,
) -> OSResult<AlbumID3WithSongs> {
    let index = state.index().await;
    let ratings = state.ratings().await;

    let album = index
        .albums
        .get(&id)
        .ok_or((StatusCode::NOT_FOUND, "Provided album ID was not found"))?;

    Ok(OSNestedResponse(
        f,
        "album",
        album_to_id3_with_songs(album, &index, &ratings),
    ))
}

#[derive(Deserialize)]
struct GetSongParams {
    id: TrackID,
}

async fn get_song(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    Query(GetSongParams { id }): Query<GetSongParams>,
    State(state): State<HttpState>,
) -> OSResult<Child> {
    let index = state.index().await;

    let track = index
        .tracks
        .get(&id)
        .ok_or((StatusCode::NOT_FOUND, "Provided track ID was not found"))?;

    let ratings = state.ratings().await;

    Ok(OSNestedResponse(
        f,
        "song",
        track_to_child(track, &index, &ratings),
    ))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetArtistInfo2Params {
    #[serde(rename = "id")]
    artist_id: ArtistID,
    // #[serde(rename = "count")]
    // similar_artists_count: Option<usize>, (TODO)
    // include_not_present: Option<bool>, (TODO)
}

async fn get_artist_info2(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    Query(GetArtistInfo2Params { artist_id }): Query<GetArtistInfo2Params>,
    State(state): State<HttpState>,
) -> OSResult<ArtistInfo2> {
    let index = state.index().await;

    if !index.artists.contains_key(&artist_id) {
        return Err((
            StatusCode::NOT_FOUND,
            "The provided artist ID was not found",
        ));
    }

    let get_image_uri =
        |art_size: ArtSize| make_cover_art_uri(CoverArtId::Artist(artist_id), art_size);

    Ok(OSNestedResponse(
        f,
        "artistInfo2",
        ArtistInfo2 {
            biography: None,
            music_brainz_id: None,
            last_fm_url: None,
            small_image_url: Some(get_image_uri(ArtSize::Small)),
            medium_image_url: Some(get_image_uri(ArtSize::Medium)),
            large_image_url: Some(get_image_uri(ArtSize::Large)),
            similar_artists: None, // TODO
        },
    ))
}

#[derive(Deserialize)]
struct GetAlbumInfo2Params {
    #[serde(rename = "id")]
    album_id: AlbumID,
}

async fn get_album_info2(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    Query(GetAlbumInfo2Params { album_id }): Query<GetAlbumInfo2Params>,
    State(state): State<HttpState>,
) -> OSResult<AlbumInfo> {
    let index = state.index().await;

    if !index.albums.contains_key(&album_id) {
        return Err((StatusCode::NOT_FOUND, "The provided album ID was not found"));
    }

    let get_image_uri =
        |art_size: ArtSize| make_cover_art_uri(CoverArtId::Album(album_id), art_size);

    Ok(OSNestedResponse(
        f,
        "albumInfo",
        AlbumInfo {
            notes: None,
            music_brainz_id: None,
            last_fm_url: None,
            small_image_url: Some(get_image_uri(ArtSize::Small)),
            medium_image_url: Some(get_image_uri(ArtSize::Medium)),
            large_image_url: Some(get_image_uri(ArtSize::Large)),
        },
    ))
}

use std::sync::Arc;

use axum::{Extension, extract::Query, http::StatusCode};
use serde::Deserialize;

use crate::{
    http::{
        HttpState,
        opensubsonic::{
            convert::{album_to_id3_with_songs, track_to_child},
            types::{
                AlbumInfo, ArtistInfo2, Child, CoverArtId, Genre, MUSIC_FOLDER_ID, MusicFolder,
            },
        },
    },
    index::{AlbumID, ArtistID, TrackID},
    os_struct,
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
    Extension(state): Extension<Arc<HttpState>>,
) -> OSNestedResponse<GetGenresAnswer> {
    let index = state.index.read().await;

    OSNestedResponse(
        f,
        "genres",
        GetGenresAnswer {
            genre: index
                .cache
                .genre_infos
                .iter()
                .map(|(id, genre)| {
                    let tracks = index.cache.genres_tracks.get(id).unwrap();
                    let albums = index.cache.genres_albums.get(id).unwrap();

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
    Extension(state): Extension<Arc<HttpState>>,
) -> OSResult<AlbumID3WithSongs> {
    let index = state.index.read().await;
    let user_data = state.user_data.read().await;

    let album = index
        .cache
        .albums_infos
        .get(&id)
        .ok_or((StatusCode::NOT_FOUND, "Provided album ID was not found"))?;

    Ok(OSNestedResponse(
        f,
        "album",
        album_to_id3_with_songs(album, &index, Some(&user_data)),
    ))
}

#[derive(Deserialize)]
struct GetSongParams {
    id: TrackID,
}

async fn get_song(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    Query(GetSongParams { id }): Query<GetSongParams>,
    Extension(state): Extension<Arc<HttpState>>,
) -> OSResult<Child> {
    let index = state.index.read().await;

    let track = index
        .tracks
        .get(&id)
        .ok_or((StatusCode::NOT_FOUND, "Provided track ID was not found"))?;

    let user_data = state.user_data.read().await;

    Ok(OSNestedResponse(
        f,
        "song",
        track_to_child(track, &index, &user_data),
    ))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetArtistInfo2Params {
    #[serde(rename = "id")]
    artist_id: ArtistID,
    #[serde(rename = "count")]
    similar_artists_count: Option<usize>,
    include_not_present: Option<bool>,
}

async fn get_artist_info2(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    Query(GetArtistInfo2Params {
        artist_id,
        similar_artists_count,
        include_not_present,
    }): Query<GetArtistInfo2Params>,
    Extension(state): Extension<Arc<HttpState>>,
) -> OSResult<ArtistInfo2> {
    let index = state.index.read().await;

    let artist = index.cache.artists_infos.get(&artist_id).ok_or((
        StatusCode::NOT_FOUND,
        "The provided artist ID was not found",
    ))?;

    let img_url = state
        .resource_manager
        .artist_art_path(artist_id)
        .map(|_| make_cover_art_uri(CoverArtId::Artist(artist.get_id())));

    Ok(OSNestedResponse(
        f,
        "artistInfo2",
        ArtistInfo2 {
            biography: None,
            music_brainz_id: None,
            last_fm_url: None,
            // TODO: different image size
            small_image_url: img_url.clone(),
            medium_image_url: img_url.clone(),
            large_image_url: img_url,
            similar_artists: None, // TODO
        },
    ))
}

#[derive(Deserialize)]
struct GetAlbumInfo2Params {
    id: AlbumID,
}

async fn get_album_info2(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    Query(GetAlbumInfo2Params { id }): Query<GetAlbumInfo2Params>,
    Extension(state): Extension<Arc<HttpState>>,
) -> OSResult<AlbumInfo> {
    let index = state.index.read().await;

    let album = index
        .cache
        .albums_infos
        .get(&id)
        .ok_or((StatusCode::NOT_FOUND, "The provided album ID was not found"))?;

    let img_url = make_cover_art_uri(CoverArtId::Album(album.get_id()));

    Ok(OSNestedResponse(
        f,
        "albumInfo",
        AlbumInfo {
            notes: None,
            music_brainz_id: None,
            last_fm_url: None,
            // TODO: different image size
            small_image_url: Some(img_url.clone()),
            medium_image_url: Some(img_url.clone()),
            large_image_url: Some(img_url),
        },
    ))
}

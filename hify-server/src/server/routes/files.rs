use axum::{
    Router,
    body::Body,
    extract::{Path, Query, State},
    http::{Request, Response, StatusCode},
    routing::get,
};
use serde::Deserialize;
use tower_http::services::fs::ServeFileSystemResponseBody;

use crate::{
    arts::ArtSize,
    index::{AlbumID, ArtistID, GenreID, TrackID},
    manager::Entity,
    server::{
        HttpState,
        utils::files::{ServedFile, serve_file},
    },
};

#[rustfmt::skip]
pub fn router() -> Router<HttpState> {
    Router::new()
        .route("/artist/{id}/art", get(artist_art))
        .route("/album/{id}/art", get(album_art))
        .route("/genre/{id}/art", get(genre_art))
        .route("/track/{id}/audio", get(track_audio_file))
}

async fn artist_art(
    State(state): State<HttpState>,
    Path(artist_id): Path<ArtistID>,
    Query(ArtSizeQuery { size: art_size }): Query<ArtSizeQuery>,
    req: Request<Body>,
) -> Result<Response<ServeFileSystemResponseBody>, (StatusCode, &'static str)> {
    if !state.index().await.artists.contains_key(&artist_id) {
        return Err((StatusCode::NOT_FOUND, "Provided artist was not found"));
    }

    let art_path = state.get_art(Entity::Artist(artist_id), art_size).unwrap();

    Ok(serve_file(&art_path, req).await)
}

async fn album_art(
    State(state): State<HttpState>,
    Path(album_id): Path<AlbumID>,
    Query(ArtSizeQuery { size: art_size }): Query<ArtSizeQuery>,
    req: Request<Body>,
) -> Result<ServedFile, (StatusCode, &'static str)> {
    if !state.index().await.albums.contains_key(&album_id) {
        return Err((StatusCode::NOT_FOUND, "Provided album was not found"));
    }

    let art_path = state.get_art(Entity::Album(album_id), art_size).unwrap();

    Ok(serve_file(&art_path, req).await)
}

async fn genre_art(
    State(state): State<HttpState>,
    Path(genre_id): Path<GenreID>,
    Query(ArtSizeQuery { size: art_size }): Query<ArtSizeQuery>,
    req: Request<Body>,
) -> Result<Response<ServeFileSystemResponseBody>, (StatusCode, &'static str)> {
    if !state.index().await.genres.contains_key(&genre_id) {
        return Err((StatusCode::NOT_FOUND, "Provided genre was not found"));
    }

    let art_path = state.get_art(Entity::Genre(genre_id), art_size).unwrap();

    Ok(serve_file(&art_path, req).await)
}

#[derive(Deserialize)]
struct ArtSizeQuery {
    size: ArtSize,
}

async fn track_audio_file(
    State(state): State<HttpState>,
    Path(track_id): Path<TrackID>,
    req: Request<Body>,
) -> Result<Response<ServeFileSystemResponseBody>, (StatusCode, &'static str)> {
    let index = state.index().await;

    let track = index
        .tracks
        .get(&track_id)
        .ok_or((StatusCode::NOT_FOUND, "Provided track was not found"))?;

    Ok(serve_file(&state.music_dir().join(&track.relative_path), req).await)
}

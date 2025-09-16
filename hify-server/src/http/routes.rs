use std::sync::Arc;

use axum::{
    Extension,
    body::Body,
    extract::Path,
    http::{Request, Response, StatusCode},
};
use tower::ServiceExt;
use tower_http::services::{ServeFile, fs::ServeFileSystemResponseBody};

use crate::index::{AlbumID, ArtistID, TrackID};

use super::HttpState;

pub async fn album_art(
    Extension(state): Extension<Arc<HttpState>>,
    Path(album_id): Path<AlbumID>,
    req: Request<Body>,
) -> Result<Response<ServeFileSystemResponseBody>, (StatusCode, &'static str)> {
    // TODO: different sizes
    let art_path = state
        .resource_manager
        .album_arts
        .large_art(album_id)
        .ok_or((StatusCode::NOT_FOUND, "Provided album art was not found"))?;

    // NOTE: The `ServeFile` service may produce an error, but will return it as an Ok() value
    let served = ServeFile::new(art_path)
        .oneshot(req)
        .await
        // We can unwrap as the Err() variant is Infallible
        .unwrap();

    Ok(served)
}

pub async fn artist_art(
    Extension(state): Extension<Arc<HttpState>>,
    Path(artist_id): Path<ArtistID>,
    req: Request<Body>,
) -> Result<Response<ServeFileSystemResponseBody>, (StatusCode, &'static str)> {
    // TODO: different sizes
    let art_path = state
        .resource_manager
        .artist_arts
        .large_art(artist_id)
        .ok_or((
            StatusCode::NOT_FOUND,
            "The provided artist does not have an associated art",
        ))?;

    // NOTE: The `ServeFile` service may produce an error, but will return it as an Ok() value
    let served = ServeFile::new(art_path)
        .oneshot(req)
        .await
        // We can unwrap as the Err() variant is Infallible
        .unwrap();

    Ok(served)
}

pub async fn stream(
    Extension(state): Extension<Arc<HttpState>>,
    Path(track_id): Path<TrackID>,
    req: Request<Body>,
) -> Result<Response<ServeFileSystemResponseBody>, (StatusCode, &'static str)> {
    let index = state.index.read().await;

    let track = index
        .tracks
        .get(&track_id)
        .ok_or((StatusCode::NOT_FOUND, "Provided track was not found"))?;

    // NOTE: The `ServeFile` service may produce an error, but will return it as an Ok() value
    let served = ServeFile::new(state.music_dir.join(&track.relative_path))
        .oneshot(req)
        .await
        // We can unwrap as the Err() variant is Infallible
        .unwrap();

    Ok(served)
}

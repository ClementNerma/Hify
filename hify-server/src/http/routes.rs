use std::sync::Arc;

use axum::{
    Extension,
    body::Body,
    extract::Path,
    http::{Request, Response, StatusCode},
};
use serde::Deserialize;
use tower::ServiceExt;
use tower_http::services::{ServeFile, fs::ServeFileSystemResponseBody};

use crate::index::{AlbumID, ArtistID, TrackID};

use super::HttpState;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArtSize {
    Large,
    Medium,
    Small,
}

pub async fn album_art(
    Extension(state): Extension<Arc<HttpState>>,
    Path((album_id, art_size)): Path<(AlbumID, ArtSize)>,
    req: Request<Body>,
) -> Result<Response<ServeFileSystemResponseBody>, (StatusCode, &'static str)> {
    let arts_manager = &state.resource_manager.album_arts;

    let art_path = match art_size {
        ArtSize::Large => arts_manager.large_art(album_id),
        ArtSize::Medium => arts_manager.medium_art(album_id),
        ArtSize::Small => arts_manager.small_art(album_id),
    };

    let art_path = art_path.ok_or((StatusCode::NOT_FOUND, "Provided album art was not found"))?;

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
    Path((artist_id, art_size)): Path<(ArtistID, ArtSize)>,
    req: Request<Body>,
) -> Result<Response<ServeFileSystemResponseBody>, (StatusCode, &'static str)> {
    let arts_manager = &state.resource_manager.artist_arts;

    let art_path = match art_size {
        ArtSize::Large => arts_manager.large_art(artist_id),
        ArtSize::Medium => arts_manager.medium_art(artist_id),
        ArtSize::Small => arts_manager.small_art(artist_id),
    };

    let art_path = art_path.ok_or((
        StatusCode::NOT_FOUND,
        "The provided artist does not have associated arts",
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

use std::sync::Arc;

use axum::{
    body::Body,
    extract::Path,
    http::{Request, StatusCode},
    response::IntoResponse,
    Extension,
};
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::{
    index::{AlbumID, ArtistID, IdType, TrackID},
    resources::ArtistArt,
};

use super::AppState;

pub async fn album_art(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
    req: Request<Body>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let id =
        AlbumID::decode(&id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid album ID provided"))?;

    let index = state.index.read().await;

    let relative_path = index
        .album_arts
        .get(&id)
        .ok_or((StatusCode::NOT_FOUND, "Provided album art was not found"))?;

    // NOTE: The `ServeFile` service may produce an error, but will return it as an Ok() value
    let served = ServeFile::new(index.from.join(relative_path))
        .oneshot(req)
        .await
        // We can unwrap as the Err() variant is Infallible
        .unwrap();

    Ok(served)
}

pub async fn artist_art(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
    req: Request<Body>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let artist_id = ArtistID::decode(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid artist ID provided"))?;

    let art_path = state
        .resource_manager
        .get_path_of::<ArtistArt>(artist_id)
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
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
    req: Request<Body>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let id =
        TrackID::decode(&id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid track ID provided"))?;

    let index = state.index.read().await;

    let track = index
        .tracks
        .get(&id)
        .ok_or((StatusCode::NOT_FOUND, "Provided track was not found"))?;

    // NOTE: The `ServeFile` service may produce an error, but will return it as an Ok() value
    let served = ServeFile::new(index.from.join(&track.relative_path))
        .oneshot(req)
        .await
        // We can unwrap as the Err() variant is Infallible
        .unwrap();

    Ok(served)
}

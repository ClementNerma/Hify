use axum::{
    body::{Body, StreamBody},
    extract::Path,
    http::{Request, StatusCode},
    response::IntoResponse,
    Extension,
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::index::{ArtID, ArtTarget, ArtistID, TrackID};

use super::AppState;

pub async fn art(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let id =
        ArtID::decode(&id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid art ID provided"))?;

    let index = state.index.read().await;

    let art = index
        .arts
        .get(&id)
        .ok_or((StatusCode::NOT_FOUND, "Provided art was not found"))?;

    // Cannot fail given we only look for art files with specific file extensions
    let ext = art.relative_path.extension().unwrap().to_str().unwrap();

    let mime_type = mime_guess::from_ext(ext).first_raw().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal error: failed to determine MIME type for art file",
    ))?;

    let file = File::open(index.from.join(&art.relative_path))
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal error: failed to open the art file",
            )
        })?;

    let body = StreamBody::new(ReaderStream::new(file));

    Ok(([("Content-Type", mime_type)], body))
}

pub async fn artist_art(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let id = ArtistID::decode(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid artist ID provided"))?;

    let index = state.index.read().await;

    let artist_albums = index
        .cache
        .artists_albums
        .get(&id)
        .ok_or((StatusCode::NOT_FOUND, "Provided artist was not found"))?;

    let artist_first_album_id = artist_albums.keys().next().ok_or((
        StatusCode::NOT_FOUND,
        "Provided artist does not have any album to generate art from",
    ))?;

    let album_art = index
        .arts
        .get(&ArtTarget::AlbumCover(*artist_first_album_id).to_id())
        .ok_or((
            StatusCode::NOT_FOUND,
            "Artist's first album does not have a cover art",
        ))?;

    // TODO: improve artist arts
    let art = album_art;

    // Cannot fail given we only look for art files with specific file extensions
    let ext = art.relative_path.extension().unwrap().to_str().unwrap();

    let mime_type = mime_guess::from_ext(ext).first_raw().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal error: failed to determine MIME type for art file",
    ))?;

    let file = File::open(index.from.join(&art.relative_path))
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal error: failed to open the art file",
            )
        })?;

    let body = StreamBody::new(ReaderStream::new(file));

    Ok(([("Content-Type", mime_type)], body))
}

pub async fn stream(
    Extension(state): Extension<AppState>,
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
    let served = ServeFile::new(
        index
            .from
            .join(&format!("{}a", track.relative_path.display())),
    )
    .oneshot(req)
    .await
    // We can unwrap as the Err() variant is Infallible
    .unwrap();

    Ok(served)
}

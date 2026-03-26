use anyhow::Context;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{post, put},
};
use serde::Deserialize;
use tokio::task::spawn_blocking;

use crate::{
    index::{Rating, TrackID},
    server::{
        HttpState,
        utils::response::{ApiResponse, ApiResult},
    },
};

#[rustfmt::skip]
pub fn router() -> Router<HttpState> {
    Router::new()
        .route("/index/update", post(update_index))
        .route("/tracks/{id}/rating", put(set_track_rating).delete(remove_track_rating))
}

async fn update_index(State(state): State<HttpState>) -> ApiResult<()> {
    spawn_blocking(move || state.update_index())
        .await
        .context("Failed to update index")?
        .context("Failed to update index")?;

    Ok(ApiResponse(()))
}

async fn set_track_rating(
    State(state): State<HttpState>,
    Path(track_id): Path<TrackID>,
    Json(payload): Json<SetRatingPayload>,
) -> ApiResult<()> {
    state
        .set_track_rating(track_id, payload.rating)
        .await
        .with_context(|| format!("Failed to set rating for track ID {track_id:?}"))?;

    Ok(ApiResponse(()))
}

#[derive(Deserialize)]
struct SetRatingPayload {
    rating: Rating,
}

async fn remove_track_rating(
    State(state): State<HttpState>,
    Path(track_id): Path<TrackID>,
) -> ApiResult<()> {
    state
        .remove_track_rating(track_id)
        .await
        .with_context(|| format!("Failed to set rating for track ID {track_id:?}"))?;

    Ok(ApiResponse(()))
}

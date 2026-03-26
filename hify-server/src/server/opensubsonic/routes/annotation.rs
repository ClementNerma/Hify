use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use serde::Deserialize;

use crate::{
    index::Rating,
    server::{
        HttpState,
        opensubsonic::{OSCommonParams, OSEmptyResponse, OSError, types::CoverArtId},
    },
};

use super::OpenSubsonicRouter;

pub fn router() -> OpenSubsonicRouter {
    OpenSubsonicRouter::new()
        .route("/setRating", set_rating)
        .route("/scrobble", scrobble)
}

#[derive(Deserialize)]
pub struct SetRatingParams {
    id: CoverArtId, // May be an album or artist (folder) ID
    rating: u8,
}

async fn set_rating(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    Query(SetRatingParams { id, rating }): Query<SetRatingParams>,
    State(state): State<HttpState>,
) -> Result<OSEmptyResponse, OSError> {
    let rating = Rating::try_from(rating * 2)
        .map_err(|()| (StatusCode::BAD_REQUEST, "Invalid rating provided"))?;

    let index = state.index().await;

    match id {
        CoverArtId::Track(track_id) => {
            if !index.tracks.contains_key(&track_id) {
                return Err((StatusCode::NOT_FOUND, "Provided ID was not found"));
            }

            state
                .set_track_rating(track_id, rating)
                // TODO: pass error message to returner
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to set rating"))?;

            Ok(OSEmptyResponse(f))
        }

        CoverArtId::Album(_) => Err((StatusCode::NOT_IMPLEMENTED, "TODO: albums")),

        CoverArtId::Artist(_) => Err((StatusCode::NOT_IMPLEMENTED, "TODO: artists")),
    }
}

// (TODO?) Scrobbling is not supported, so a placeholder handler is put here
async fn scrobble(Query(OSCommonParams { f }): Query<OSCommonParams>) -> OSEmptyResponse {
    OSEmptyResponse(f)
}

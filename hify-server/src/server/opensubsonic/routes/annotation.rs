use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use serde::Deserialize;

use crate::{
    index::Rating,
    server::{
        HttpState,
        opensubsonic::{OSEmptyResponse, OSError, types::CoverArtId},
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
    Query(SetRatingParams { id, rating }): Query<SetRatingParams>,
    State(state): State<HttpState>,
) -> Result<OSEmptyResponse, OSError> {
    let rating = if rating != 0 {
        Some(
            Rating::try_from(rating)
                .map_err(|()| (StatusCode::BAD_REQUEST, "Invalid rating provided"))?,
        )
    } else {
        None
    };

    let index = state.index().await;

    match id {
        CoverArtId::Track(track_id) => {
            if !index.tracks.contains_key(&track_id) {
                return Err((StatusCode::NOT_FOUND, "Provided ID was not found"));
            }

            match rating {
                Some(rating) => {
                    state
                        .set_track_rating(track_id, rating)
                        // TODO: pass error message to returner
                        .await
                        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to set rating"))?;
                }

                None => {
                    state.remove_track_rating(track_id).await.map_err(|_| {
                        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to remove rating")
                    })?;
                }
            }

            Ok(OSEmptyResponse)
        }

        CoverArtId::Album(_) => Err((StatusCode::NOT_IMPLEMENTED, "TODO: albums")),

        CoverArtId::Artist(_) => Err((StatusCode::NOT_IMPLEMENTED, "TODO: artists")),
    }
}

// (TODO?) Scrobbling is not supported, so a placeholder handler is put here
async fn scrobble() -> OSEmptyResponse {
    OSEmptyResponse
}

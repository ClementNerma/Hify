use std::sync::Arc;

use axum::{Extension, extract::Query, http::StatusCode};
use serde::Deserialize;

use crate::{
    http::{
        HttpState,
        opensubsonic::{OSCommonParams, OSEmptyResponse, OSError, types::CoverArtId},
    },
    index::Rating,
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
    Extension(state): Extension<Arc<HttpState>>,
) -> Result<OSEmptyResponse, OSError> {
    let rating = match rating {
        0 => None,
        1 => Some(Rating::Two),
        2 => Some(Rating::Four),
        3 => Some(Rating::Six),
        4 => Some(Rating::Eight),
        5 => Some(Rating::Ten),
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid rating provided")),
    };

    let index = state.index.read().await;

    match id {
        CoverArtId::Track(track_id) => {
            if !index.tracks.contains_key(&track_id) {
                return Err((StatusCode::NOT_FOUND, "Provided ID was not found"));
            }

            let mut user_data = state.user_data.write().await;
            user_data.set_track_rating(track_id, rating);

            Ok(OSEmptyResponse(f))
        }

        CoverArtId::Album(_) => return Err((StatusCode::NOT_IMPLEMENTED, "TODO")),

        CoverArtId::Artist(_) => return Err((StatusCode::NOT_IMPLEMENTED, "TODO")),
    }
}

// (TODO?) Scrobbling is not supported, so a placeholder handler is put here
async fn scrobble(Query(OSCommonParams { f }): Query<OSCommonParams>) -> OSEmptyResponse {
    OSEmptyResponse(f)
}

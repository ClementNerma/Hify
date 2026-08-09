use axum::extract::{Query, State};
use serde::Deserialize;

use crate::{
    index::Rating,
    server::{
        HttpState,
        opensubsonic::{OSEmptyResponse, OSError, OSResult, types::CoverArtId},
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
) -> OSResult<OSEmptyResponse> {
    let rating = if rating != 0 {
        Some(Rating::try_from(rating).map_err(|()| "Invalid rating provided")?)
    } else {
        None
    };

    let index = state.index().await;

    match id {
        CoverArtId::Track(track_id) => {
            if !index.tracks.contains_key(&track_id) {
                return Err(OSError("Provided ID was not found"));
            }

            match rating {
                Some(rating) => {
                    state
                        .set_track_rating(track_id, rating)
                        // TODO: pass error message to returner
                        .await
                        .map_err(|_| "Failed to set rating")?;
                }

                None => {
                    state
                        .remove_track_rating(track_id)
                        .await
                        .map_err(|_| "Failed to remove rating")?;
                }
            }

            Ok(OSEmptyResponse)
        }

        CoverArtId::Album(_) => Err(OSError("TODO: albums")),

        CoverArtId::Artist(_) => Err(OSError("TODO: artists")),
    }
}

// (TODO?) Scrobbling is not supported, so a placeholder handler is put here
async fn scrobble() -> OSEmptyResponse {
    OSEmptyResponse
}

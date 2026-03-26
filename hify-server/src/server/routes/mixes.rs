use anyhow::Context;
use axum::{
    Router,
    extract::{Query, State},
    routing::get,
};
use serde::Deserialize;

use crate::server::{
    HttpState,
    utils::{
        dtos::TrackCompleteInfos,
        mixes,
        pagination::{Paginated, Pagination, PaginationDir},
        response::{ApiResponse, ApiResult},
    },
};

#[rustfmt::skip]
pub fn router() -> Router<HttpState> {
    Router::new()
        .route("/mix", get(generate_mix))
}

async fn generate_mix(
    State(state): State<HttpState>,
    Query(params): Query<MixGenerationQuery>,
) -> ApiResult<Paginated<TrackCompleteInfos>> {
    let MixGenerationQuery {
        mix_params,
        limit,
        offset,
    } = params;

    let mix_params =
        serde_json::from_str(&mix_params).context("Failed to parse mix parameters from query")?;

    let index = state.index().await;
    let ratings = state.ratings().await;

    Ok(ApiResponse(mixes::generate_mix(
        mix_params,
        &index,
        &ratings,
        Pagination {
            limit,
            offset,
            dir: PaginationDir::Asc,
        },
    )))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MixGenerationQuery {
    // Cannot be parsed from a complex type (only types that can be represented in a query string)
    mix_params: String,

    // Cannot flatten [`Pagination`] because of axum limitations
    // See https://github.com/serde-rs/serde/issues/1183
    limit: usize,
    offset: Option<usize>,
}

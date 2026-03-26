use axum::{
    Router,
    extract::{Query, State},
    routing::get,
};
use serde::Deserialize;

use crate::server::{
    HttpState,
    utils::{
        dtos::{AlbumCompleteInfos, ArtistCompleteInfos, TrackCompleteInfos},
        pagination::{Paginated, Pagination, PaginationDir},
        response::ApiResponse,
        search,
    },
};

#[rustfmt::skip]
pub fn router() -> Router<HttpState> {
    Router::new()
        .route("/tracks/search", get(search_tracks))
        .route("/albums/search", get(search_albums))
        .route("/artists/search", get(search_artists))
}

async fn search_tracks(
    State(state): State<HttpState>,
    Query(query): Query<SearchQuery>,
) -> ApiResponse<Paginated<TrackCompleteInfos>> {
    let SearchQuery {
        query,
        limit,
        offset,
        dir,
    } = query;

    let index = state.index().await;
    let ratings = state.ratings().await;

    ApiResponse(
        search::search_tracks(&query, Pagination { limit, offset, dir }, &index, &ratings)
            .map(|track| TrackCompleteInfos::new(track, &index, &ratings)),
    )
}

async fn search_albums(
    State(state): State<HttpState>,
    Query(query): Query<SearchQuery>,
) -> ApiResponse<Paginated<AlbumCompleteInfos>> {
    let SearchQuery {
        query,
        limit,
        offset,
        dir,
    } = query;

    let index = state.index().await;

    ApiResponse(
        search::search_albums(&query, Pagination { limit, offset, dir }, &index)
            .map(|album| AlbumCompleteInfos::new(album, &index)),
    )
}

async fn search_artists(
    State(state): State<HttpState>,
    Query(query): Query<SearchQuery>,
) -> ApiResponse<Paginated<ArtistCompleteInfos>> {
    let SearchQuery {
        query,
        limit,
        offset,
        dir,
    } = query;

    let index = state.index().await;

    ApiResponse(
        search::search_artists(&query, Pagination { limit, offset, dir }, &index)
            .map(|artist| ArtistCompleteInfos::new(artist, &index)),
    )
}

#[derive(Deserialize)]
struct SearchQuery {
    query: String,

    // Cannot flatten [`Pagination`] because of axum limitations
    // See https://github.com/serde-rs/serde/issues/1183
    limit: usize,
    offset: Option<usize>,
    dir: PaginationDir,
}

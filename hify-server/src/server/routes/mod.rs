use axum::Router;

use super::HttpState;

mod files;
mod mixes;
mod mutations;
mod queries;
mod searches;

pub fn router() -> Router<HttpState> {
    Router::new()
        .route("/", axum::routing::get(async || "API is running"))
        .merge(queries::router())
        .merge(mutations::router())
        .merge(files::router())
        .merge(searches::router())
        .merge(mixes::router())
}

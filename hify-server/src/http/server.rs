use std::net::SocketAddr;

use anyhow::Result;
use axum::{middleware, routing::get, Extension, Router};
use log::info;
use tokio::net::TcpListener;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use super::{
    routes::{art, artist_art, stream},
    AppState,
};
use crate::{
    graphql::{get_graphql_schema, SaveIndexFn},
    http::{
        graphql::{graphiql, graphql_handler},
        logging::log_errors,
    },
};
use crate::{index::Index, userdata::UserDataWrapper};

pub static GRAPHQL_ENDPOINT: &str = "/graphql";

pub async fn launch(
    address: &SocketAddr,
    index: Index,
    user_data: UserDataWrapper,
    save_index: SaveIndexFn,
) -> Result<()> {
    // TODO: improve this
    let cors = CorsLayer::new()
        .allow_methods(AllowMethods::any())
        .allow_origin(AllowOrigin::any())
        .allow_headers(AllowHeaders::any());

    let app_state = AppState::new(index, user_data);

    let graphql_schema = get_graphql_schema(app_state.clone(), save_index);

    let app = Router::new()
        // Define all routes
        .route(GRAPHQL_ENDPOINT, get(graphiql).post(graphql_handler))
        .route("/art/:id", get(art))
        .route("/art/artist/:id", get(artist_art))
        .route("/stream/:id", get(stream))
        // Define extensions
        .layer(Extension(app_state))
        .layer(Extension(graphql_schema))
        // Define middlewares
        .layer(cors)
        .layer(middleware::from_fn(log_errors));

    info!("> Server is being launched on {address}");

    let listener = TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

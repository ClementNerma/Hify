use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{Extension, Router, middleware, routing::get};
use log::info;
use tokio::net::TcpListener;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use super::{
    HttpState,
    routes::{album_art, artist_art, stream},
};
use crate::{
    graphql::get_graphql_schema,
    http::{
        graphql::{graphiql, graphql_handler},
        logging::log_errors,
    },
    resources::ResourceManager,
};
use crate::{index::Index, userdata::UserDataWrapper};

pub static GRAPHQL_ENDPOINT: &str = "/graphql";

pub static OPENSUBSONIC_BASE_URI: &str = "/rest";

pub async fn launch(
    addr: SocketAddr,
    music_dir: PathBuf,
    index: Index,
    user_data: UserDataWrapper,
    res_manager: ResourceManager,
) -> Result<()> {
    // TODO: improve this
    let cors = CorsLayer::new()
        .allow_methods(AllowMethods::any())
        .allow_origin(AllowOrigin::any())
        .allow_headers(AllowHeaders::any());

    let app_state = Arc::new(HttpState::new(music_dir, index, user_data, res_manager));

    let graphql_schema = get_graphql_schema(Arc::clone(&app_state));

    let app = Router::new()
        // Define all routes
        .route(GRAPHQL_ENDPOINT, get(graphiql).post(graphql_handler))
        .route("/art/album/{id}", get(album_art))
        .route("/art/artist/{id}", get(artist_art))
        .route("/stream/{id}", get(stream))
        // Set up OpenSubsonic routes
        .nest(OPENSUBSONIC_BASE_URI, super::opensubsonic::router())
        // Define extensions
        .layer(Extension(app_state))
        .layer(Extension(graphql_schema))
        // Define middlewares
        .layer(cors)
        .layer(middleware::from_fn(log_errors));

    info!("> Server is being launched on {addr}");

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

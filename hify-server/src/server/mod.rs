use std::{net::SocketAddr, ops::Deref, sync::Arc};

use anyhow::Result;
use axum::middleware;
use colored::Colorize;
use log::info;
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
};

use crate::manager::DataManager;

use self::{routes::router, utils::logging::log_errors};

mod opensubsonic;
mod routes;
mod utils;

pub static OPENSUBSONIC_BASE_URI: &str = "/rest";

pub async fn launch(addr: SocketAddr, data_manager: DataManager) -> Result<()> {
    // TODO: improve this
    let cors = CorsLayer::new()
        .allow_methods(AllowMethods::any())
        .allow_origin(AllowOrigin::any())
        .allow_headers(AllowHeaders::any());

    // Add compression
    let compression = CompressionLayer::new().gzip(true);

    let state = HttpState(Arc::new(data_manager));

    let app = router()
        // Set up OpenSubsonic routes
        .nest(OPENSUBSONIC_BASE_URI, opensubsonic::router())
        // Set up shared state
        .with_state(state)
        // Set up CORS headers
        .layer(cors)
        // Set up compression
        .layer(compression)
        // Set up errors logging
        .layer(middleware::from_fn(log_errors));

    info!(
        "> Server is being launched on {}",
        addr.to_string().bright_green()
    );

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
struct HttpState(Arc<DataManager>);

impl Deref for HttpState {
    type Target = DataManager;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

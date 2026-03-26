use axum::{Router, handler::Handler, routing::get};

use crate::server::HttpState;

mod annotation;
mod bookmarks;
mod browsing;
mod lists;
mod media;
mod playlists;
mod searching;
mod system;

pub fn router() -> Router<HttpState> {
    Router::new()
        .merge(system::router().finish())
        .merge(browsing::router().finish())
        .merge(lists::router().finish())
        .merge(searching::router().finish())
        .merge(playlists::router().finish())
        .merge(media::router().finish())
        .merge(annotation::router().finish())
        .merge(bookmarks::router().finish())
}

struct OpenSubsonicRouter(Router<HttpState>);

impl OpenSubsonicRouter {
    pub fn new() -> Self {
        Self(Router::new())
    }

    pub fn route<T: 'static>(
        mut self,
        path: &'static str,
        handler: impl Handler<T, HttpState>,
    ) -> Self {
        self.0 = self
            .0
            .route(path, get(handler.clone()).post(handler.clone()))
            .route(&format!("{path}.view"), get(handler.clone()).post(handler));

        self
    }

    pub fn finish(self) -> Router<HttpState> {
        self.0
    }
}

use axum::{Router, handler::Handler, routing::get};

mod annotation;
mod bookmarks;
mod browsing;
mod lists;
mod media;
mod playlists;
mod searching;
mod system;

pub fn router() -> Router {
    Router::new()
        .merge(self::system::router().finish())
        .merge(self::browsing::router().finish())
        .merge(self::lists::router().finish())
        .merge(self::searching::router().finish())
        .merge(self::playlists::router().finish())
        .merge(self::media::router().finish())
        .merge(self::annotation::router().finish())
        .merge(self::bookmarks::router().finish())
}

struct OpenSubsonicRouter(Router);

impl OpenSubsonicRouter {
    pub fn new() -> Self {
        Self(Router::new())
    }

    pub fn route<T: 'static>(mut self, path: &'static str, handler: impl Handler<T, ()>) -> Self {
        self.0 = self
            .0
            .route(path, get(handler.clone()).post(handler.clone()))
            .route(&format!("{path}.view"), get(handler.clone()).post(handler));

        self
    }

    pub fn finish(self) -> Router {
        self.0
    }
}

use std::path::Path;

use axum::{body::Body, extract::Request, response::Response};
use tower::ServiceExt;
use tower_http::services::{ServeFile, fs::ServeFileSystemResponseBody};

pub type ServedFile = Response<ServeFileSystemResponseBody>;

pub async fn serve_file(path: &Path, req: Request<Body>) -> ServedFile {
    // NOTE: The `ServeFile` service may produce an error, but will return it as an Ok() value
    ServeFile::new(path)
        .oneshot(req)
        .await
        // We can unwrap as the Err() variant is Infallible
        .unwrap()
}

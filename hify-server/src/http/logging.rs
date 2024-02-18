use axum::{extract::Request, middleware::Next, response::Response};
use colored::Colorize;
use log::{debug, error};

pub async fn log_errors(request: Request, next: Next) -> Response {
    let path = request.uri().path().to_owned();

    let res = next.run(request).await;

    if res.status().is_client_error() || res.status().is_server_error() {
        error!(
            "{} {}",
            res.status().as_u16().to_string().bright_red(),
            path.bright_yellow()
        );
    } else {
        debug!(
            "{} {}",
            res.status().as_u16().to_string().bright_green(),
            path.bright_cyan()
        );
    }

    res
}

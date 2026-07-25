//!
//! This module contains a mostly-compliant [OpenSubsonic](https://opensubsonic.netlify.app/) implementation.
//!
//! **WIP:** some routes may not be available yet.
//!
//! Note that some features are explicitly out of scope, such as XML support.

mod convert;
mod routes;
mod types;

use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;

pub use self::routes::router;

/// Result of an `OpenSubsonic` request, which is either a successful response or a formatted error.
type OSResult<T> = Result<OSNestedResponse<T>, OSError>;

type OSError = (StatusCode, &'static str);

/// `OpenSubsonic` response wrapper that nests the response in the expected JSON structure.
struct OSNestedResponse<T: Serialize>(&'static str, T);

impl<T: Serialize> IntoResponse for OSNestedResponse<T> {
    fn into_response(self) -> Response {
        let Self(key, value) = self;

        let response = json!({
            "subsonic-response": {
                "status": "ok",
                "version": "1.16.1",
                "type": "HifyServer",
                "serverVersion": env!("CARGO_PKG_VERSION"),
                "openSubsonic": true,
                key: value,
            }
        });

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );

        let body = serde_json::to_string(&response).unwrap();
        (headers, body).into_response()
    }
}

/// `OpenSubsonic` response wrapper for endpoints that return no additional data.
struct OSEmptyResponse;

impl IntoResponse for OSEmptyResponse {
    fn into_response(self) -> Response {
        // TODO: DRY (above code is almost identical, but with no key/value pair)
        let response = json!({
            "subsonic-response": {
                "status": "ok",
                "version": "1.16.1",
                "type": "HifyServer",
                "serverVersion": env!("CARGO_PKG_VERSION"),
                "openSubsonic": true,
            }
        });

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );

        let body = serde_json::to_string(&response).unwrap();
        (headers, body).into_response()
    }
}

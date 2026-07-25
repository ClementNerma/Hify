//!
//! This module contains a compliant [OpenSubsonic](https://opensubsonic.netlify.app/) implementation.
//!
//! **WIP:** some routes may not be available yet.

mod convert;
mod routes;
mod types;

use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub use self::routes::router;

#[derive(Deserialize)]
pub(super) struct OSCommonParams {}

type OSResult<T> = Result<OSNestedResponse<T>, OSError>;

type OSError = (StatusCode, &'static str);

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

struct OSEmptyResponse;

impl IntoResponse for OSEmptyResponse {
    fn into_response(self) -> Response {
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

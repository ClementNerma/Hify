use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_json::json;

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

pub struct ApiResponse<T: Serialize>(pub T);

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(data) = self;

        let res = json!({
            "ok": true,
            "data": data
        });

        Json(res).into_response()
    }
}

pub struct ApiError(pub anyhow::Error);

impl ApiError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self(anyhow::anyhow!(msg.into()))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let Self(data) = self;

        let res = json!({
            "ok": false,
            "cause": format!("{data:?}")
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(res)).into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

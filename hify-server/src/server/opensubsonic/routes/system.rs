use serde::Serialize;

use crate::server::opensubsonic::{OSEmptyResponse, OSNestedResponse};

use super::OpenSubsonicRouter;

pub fn router() -> OpenSubsonicRouter {
    OpenSubsonicRouter::new()
        .route("/ping", ping)
        .route("/license", license)
    // .route("/getOpenSubsonicExtensions", get_open_subsonic_extensions) // TODO
    // .route("/tokenInfo", token_info) // TODO
}

async fn ping() -> OSEmptyResponse {
    OSEmptyResponse
}

#[derive(Serialize)]
pub struct LicenseAnswer {
    pub valid: bool,
}

async fn license() -> OSNestedResponse<LicenseAnswer> {
    OSNestedResponse("license", LicenseAnswer { valid: true })
}

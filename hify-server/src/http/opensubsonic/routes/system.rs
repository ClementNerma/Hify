use axum::extract::Query;

use crate::{
    http::opensubsonic::{OSEmptyResponse, OSNestedResponse},
    os_struct,
};

use super::{super::OSCommonParams, OpenSubsonicRouter};

pub fn router() -> OpenSubsonicRouter {
    OpenSubsonicRouter::new()
        .route("/ping", ping)
        .route("/license", license)
    // .route("/getOpenSubsonicExtensions", get_open_subsonic_extensions) // TODO
    // .route("/tokenInfo", token_info) // TODO
}

async fn ping(Query(OSCommonParams { f }): Query<OSCommonParams>) -> OSEmptyResponse {
    OSEmptyResponse(f)
}

os_struct!(
    pub struct LicenseAnswer {
        valid: bool,
    }
);

async fn license(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
) -> OSNestedResponse<LicenseAnswer> {
    OSNestedResponse(f, "license", LicenseAnswer { valid: true })
}

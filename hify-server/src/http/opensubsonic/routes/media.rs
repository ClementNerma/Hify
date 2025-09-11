use std::sync::Arc;

use axum::{
    Extension,
    extract::{Path, Query, Request},
    http::{Response, StatusCode},
};
use serde::Deserialize;
use tower_http::services::fs::ServeFileSystemResponseBody;

use crate::{
    http::{
        HttpState,
        opensubsonic::{OSError, types::CoverArtId},
        server::OPENSUBSONIC_BASE_URI,
    },
    index::TrackID,
};

use super::OpenSubsonicRouter;

pub fn router() -> OpenSubsonicRouter {
    OpenSubsonicRouter::new()
        .route("/stream", stream)
        .route(GET_COVER_ART_URI, get_cover_art)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamParams {
    id: TrackID,
    #[serde(rename = "maxBitRate")]
    max_bit_rate_kbps: Option<u32>,
    format: Option<String>,
    #[serde(rename = "time_offset")]
    time_offset_s: Option<u32>,
    // N/A: size (for videos only)
    estimate_content_length: Option<usize>,
    // N/A: converted (for videos only)
}

async fn stream(
    state: Extension<Arc<HttpState>>,
    Query(params): Query<StreamParams>,
    req: Request,
) -> Result<Response<ServeFileSystemResponseBody>, OSError> {
    let StreamParams {
        id,
        #[allow(unused_variables)] //TODO
        max_bit_rate_kbps,
        #[allow(unused_variables)] //TODO
        format,
        #[allow(unused_variables)] //TODO
        time_offset_s,
        #[allow(unused_variables)] //TODO
        estimate_content_length,
    } = params;

    crate::http::routes::stream(state, Path(id), req).await
}

static GET_COVER_ART_URI: &str = "/getCoverArt";

pub fn make_cover_art_uri(id: CoverArtId) -> String {
    format!(
        "{OPENSUBSONIC_BASE_URI}{GET_COVER_ART_URI}?id={}",
        id.encode()
    )
}

#[derive(Deserialize)]
pub struct GetCovertArtParams {
    id: String,
    size: Option<u16>,
}

async fn get_cover_art(
    state: Extension<Arc<HttpState>>,
    Query(GetCovertArtParams {
        id,
        #[allow(unused_variables)] //TODO
        size,
    }): Query<GetCovertArtParams>,
    req: Request,
) -> Result<Response<ServeFileSystemResponseBody>, OSError> {
    let id =
        CoverArtId::decode(&id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid ID provided"))?;

    match id {
        CoverArtId::Track(_) => todo!(),
        CoverArtId::Album(id) => crate::http::routes::album_art(state, Path(id), req).await,
        CoverArtId::Artist(id) => crate::http::routes::artist_art(state, Path(id), req).await,
    }
}

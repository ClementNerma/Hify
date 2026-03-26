use axum::{
    extract::{Query, Request, State},
    http::{Response, StatusCode},
};
use serde::Deserialize;
use tower_http::services::fs::ServeFileSystemResponseBody;

use crate::{
    arts::{ArtSize, LARGE_ART_SIDE_PX, MEDIUM_ART_SIDE_PX, SMALL_ART_SIDE_PX, TINY_ART_SIDE_PX},
    index::TrackID,
    manager::Entity,
    server::{
        HttpState, OPENSUBSONIC_BASE_URI,
        opensubsonic::{OSError, types::CoverArtId},
        utils::files::serve_file,
    },
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
    State(state): State<HttpState>,
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

    let index = state.index().await;

    let track = index
        .tracks
        .get(&id)
        .ok_or((StatusCode::NOT_FOUND, "Track not found"))?;

    Ok(serve_file(&state.music_dir().join(&track.relative_path), req).await)
}

static GET_COVER_ART_URI: &str = "/getCoverArt";

pub fn make_cover_art_uri(id: CoverArtId, art_size: ArtSize) -> String {
    format!(
        "{OPENSUBSONIC_BASE_URI}{GET_COVER_ART_URI}?id={}&size={}",
        id.encode(),
        match art_size {
            ArtSize::Large => LARGE_ART_SIDE_PX,
            ArtSize::Medium => MEDIUM_ART_SIDE_PX,
            ArtSize::Small => SMALL_ART_SIDE_PX,
            ArtSize::Tiny => TINY_ART_SIDE_PX,
        }
    )
}

#[derive(Deserialize)]
pub struct GetCovertArtParams {
    id: String,
    size: Option<u16>,
}

async fn get_cover_art(
    State(state): State<HttpState>,
    Query(GetCovertArtParams {
        id,
        #[allow(unused_variables)] //TODO
        size,
    }): Query<GetCovertArtParams>,
    req: Request,
) -> Result<Response<ServeFileSystemResponseBody>, OSError> {
    let id =
        CoverArtId::decode(&id).map_err(|()| (StatusCode::BAD_REQUEST, "Invalid ID provided"))?;

    let art_size = match size {
        None => ArtSize::Large,
        Some(px) => {
            if u32::from(px) < TINY_ART_SIDE_PX {
                ArtSize::Tiny
            } else if u32::from(px) < SMALL_ART_SIDE_PX {
                ArtSize::Small
            } else if u32::from(px) < MEDIUM_ART_SIDE_PX {
                ArtSize::Medium
            } else {
                ArtSize::Large
            }
        }
    };

    let index = state.index().await;

    match id {
        CoverArtId::Track(_) => todo!(),

        CoverArtId::Album(id) => {
            if !index.albums.contains_key(&id) {
                return Err((StatusCode::NOT_FOUND, "Provided album ID was not found"));
            }

            let art_path = state.get_art(Entity::Album(id), art_size).unwrap();

            Ok(serve_file(&art_path, req).await)
        }

        CoverArtId::Artist(id) => {
            if !index.artists.contains_key(&id) {
                return Err((StatusCode::NOT_FOUND, "Provided artist ID was not found"));
            }

            let art_path = state.get_art(Entity::Artist(id), art_size).unwrap();

            Ok(serve_file(&art_path, req).await)
        }
    }
}

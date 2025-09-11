use axum::extract::Query;

use crate::{
    http::opensubsonic::{types::Playlist, OSCommonParams, OSNestedResponse},
    os_struct,
};

use super::OpenSubsonicRouter;

pub fn router() -> OpenSubsonicRouter {
    OpenSubsonicRouter::new().route("/getPlaylists", get_playlists)
}

os_struct!(pub struct GetPlaylistsAnswer { #[children] { playlist: Vec<Playlist> } });

async fn get_playlists(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
    // TODO: query parameters
) -> OSNestedResponse<GetPlaylistsAnswer> {
    OSNestedResponse(f, "playlists", GetPlaylistsAnswer { playlist: vec![] })
}

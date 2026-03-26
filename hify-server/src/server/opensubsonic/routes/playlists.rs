use axum::extract::Query;

use crate::{
    os_struct,
    server::opensubsonic::{OSCommonParams, OSNestedResponse, types::Playlist},
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

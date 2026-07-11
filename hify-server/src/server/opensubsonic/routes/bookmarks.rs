use std::time::SystemTime;

use axum::extract::Query;
use serde::Serialize;

use crate::server::opensubsonic::{
    OSCommonParams, OSNestedResponse,
    convert::to_iso_8601,
    types::{Bookmark, PlayQueue},
};

use super::OpenSubsonicRouter;

pub fn router() -> OpenSubsonicRouter {
    OpenSubsonicRouter::new()
        .route("/getBookmarks", get_bookmarks)
        .route("/getPlayQueue", get_play_queue)
}

#[derive(Serialize)]
pub struct GetBookmarksAnswer {
    pub bookmark: Vec<Bookmark>,
}

async fn get_bookmarks(
    Query(OSCommonParams {}): Query<OSCommonParams>,
) -> OSNestedResponse<GetBookmarksAnswer> {
    OSNestedResponse("bookmarks", GetBookmarksAnswer { bookmark: vec![] })
}

async fn get_play_queue(
    Query(OSCommonParams {}): Query<OSCommonParams>,
) -> OSNestedResponse<PlayQueue> {
    OSNestedResponse(
        "playQueue",
        PlayQueue {
            current: None,
            position: None,
            username: "admin".to_owned(), // TODO: constant?
            changed_iso_8601: to_iso_8601(SystemTime::UNIX_EPOCH),
            changed_by_app: String::new(),
            tracks: None,
        },
    )
}

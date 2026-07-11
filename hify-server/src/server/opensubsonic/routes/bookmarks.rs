use std::time::SystemTime;

use axum::extract::Query;

use crate::{
    os_struct,
    server::opensubsonic::{
        OSCommonParams, OSNestedResponse,
        convert::to_iso_8601,
        types::{Bookmark, PlayQueue},
    },
};

use super::OpenSubsonicRouter;

pub fn router() -> OpenSubsonicRouter {
    OpenSubsonicRouter::new()
        .route("/getBookmarks", get_bookmarks)
        .route("/getPlayQueue", get_play_queue)
}

os_struct! {
    pub struct GetBookmarksAnswer {
        #[children] {
            bookmark: Vec<Bookmark>
        }
    }
}

async fn get_bookmarks(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
) -> OSNestedResponse<GetBookmarksAnswer> {
    OSNestedResponse(f, "bookmarks", GetBookmarksAnswer { bookmark: vec![] })
}

async fn get_play_queue(
    Query(OSCommonParams { f }): Query<OSCommonParams>,
) -> OSNestedResponse<PlayQueue> {
    OSNestedResponse(
        f,
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

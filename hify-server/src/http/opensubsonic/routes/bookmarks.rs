use axum::extract::Query;

use crate::{
    http::opensubsonic::{types::Bookmark, OSCommonParams, OSNestedResponse},
    os_struct,
};

use super::OpenSubsonicRouter;

pub fn router() -> OpenSubsonicRouter {
    OpenSubsonicRouter::new().route("/getBookmarks", get_bookmarks)
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

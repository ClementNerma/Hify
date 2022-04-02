use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

use super::server::GRAPHQL_MOUNTPOINT;

pub struct CachingStrategy;

#[rocket::async_trait]
impl Fairing for CachingStrategy {
    fn info(&self) -> Info {
        Info {
            name: "Enable browser-side caching",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // Disable caching for GraphQL
        if request.uri().path() == GRAPHQL_MOUNTPOINT {
            response.set_header(Header::new(
                "Cache-Control",
                "no-cache, no-store, must-revalidate",
            ));
            response.set_header(Header::new("Pragma", "no-cache"));
            response.set_header(Header::new("Expires", "0"));
            return;
        }

        response.set_header(Header::new("Cache-Control", "private, max-age=2678400"));
    }
}

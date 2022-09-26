use rocket::Rocket;
use rocket::{http::Status, Ignite};
use serde::Serialize;

use rocket::response::status;

use super::{
    cache::CachingStrategy,
    cors::CORS,
    routes::{album_art, artist_art, exit, stream},
    AppState,
};
use crate::graphql::{get_graphql_routes, get_graphql_schema, SaveIndexFn};
use crate::{index::Index, userdata::UserDataWrapper};

pub static GRAPHQL_MOUNTPOINT: &str = "/graphql";

pub async fn launch(
    index: Index,
    user_data: UserDataWrapper,
    save_index: SaveIndexFn,
) -> Result<Rocket<Ignite>, rocket::Error> {
    let app_state = AppState::new(index, user_data);

    Rocket::build()
        .attach(CORS)
        .attach(CachingStrategy)
        .manage(get_graphql_schema(app_state.clone(), save_index))
        .manage(app_state)
        .mount(GRAPHQL_MOUNTPOINT, get_graphql_routes())
        .mount("/", rocket::routes![album_art, artist_art, stream, exit])
        .launch()
        .await
}

pub fn rest_server_error(status: Status, message: String) -> status::Custom<String> {
    status::Custom(
        status,
        serde_json::to_string(&ServerError { message }).unwrap(),
    )
}

#[derive(Serialize)]
struct ServerError {
    message: String,
}

pub type FaillibleResponse<T> = Result<T, status::Custom<String>>;

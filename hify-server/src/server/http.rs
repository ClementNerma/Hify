use std::path::Path;

use rocket::{
    http::{ContentType, Status},
    response::{content::Custom, status},
    tokio::fs::File,
    Rocket, State,
};
use serde::Serialize;

use super::{
    cors::CORS,
    graphql::{get_graphql_routes, get_graphql_schema, AppSchema},
};
use crate::index::{AudioFormat, Index, TrackID};

pub async fn launch(index: Index) -> Result<(), rocket::Error> {
    Rocket::build()
        .attach(CORS)
        .manage(get_graphql_schema(index))
        .mount("/graphql", get_graphql_routes())
        .mount("/", rocket::routes![stream])
        .launch()
        .await
}

fn rest_server_error(status: Status, message: String) -> status::Custom<String> {
    status::Custom(
        status,
        serde_json::to_string(&ServerError { message }).unwrap(),
    )
}

#[derive(Serialize)]
struct ServerError {
    message: String,
}

type FaillibleResponse<T> = Result<T, status::Custom<String>>;

#[rocket::get("/stream/<id>")]
pub async fn stream(ctx: &State<AppSchema>, id: String) -> FaillibleResponse<Custom<File>> {
    Err(rest_server_error(
        Status::NotImplemented,
        "Not currently supported".to_string(),
    ))
    // let track_path = cache
    //     .tracks_paths
    //     .get(&TrackID(id.clone()))
    //     .ok_or_else(|| {
    //         rest_server_error(
    //             Status::NotFound,
    //             "Provided track ID was not found".to_string(),
    //         )
    //     })?;
    // let track_format = cache.tracks_formats.get(&TrackID(id)).unwrap();

    // let file = File::open(Path::new(track_path)).await.map_err(|err| {
    //     rest_server_error(
    //         Status::InternalServerError,
    //         format!("Failed to open track file: {err}"),
    //     )
    // })?;

    // let mime_type = match track_format {
    //     AudioFormat::MP3 => ContentType::MPEG,
    //     AudioFormat::FLAC => ContentType::FLAC,
    // };

    // Ok(Custom(mime_type, file))
}

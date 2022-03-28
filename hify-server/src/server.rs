use rocket::{
    http::{ContentType, Status},
    response::{
        content::{Custom, Json},
        status,
    },
    routes,
    tokio::{fs::File, sync::RwLock},
    State as RocketState,
};
use serde::Serialize;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    builder::build_index,
    index::{AudioFormat, Library},
};

type State = RocketState<ServerState>;

pub struct ServerState {
    root_path: PathBuf,
    index: Arc<RwLock<Option<Library>>>,
}

#[derive(Serialize)]
struct ServerError {
    message: String,
}

type FaillibleResponse<T> = Result<T, status::Custom<String>>;

fn server_error(status: Status, message: String) -> status::Custom<String> {
    status::Custom(
        status,
        serde_json::to_string(&ServerError { message }).unwrap(),
    )
}

fn mime_type(audio_format: AudioFormat) -> ContentType {
    match audio_format {
        AudioFormat::MP3 => ContentType::MPEG,
        AudioFormat::FLAC => ContentType::FLAC,
    }
}

#[get("/version")]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[get("/index?<since>")]
async fn index(state: &State, since: u64) -> FaillibleResponse<String> {
    let index = state.index.read().await;

    match *index {
        Some(ref index) => {
            if index.creation_time != since {
                Ok(serde_json::to_string(index).unwrap())
            } else {
                Err(server_error(
                    Status::NotModified,
                    "State didn't change".to_string(),
                ))
            }
        }
        None => Err(server_error(
            Status::NotFound,
            "Please generate a state first".to_string(),
        )),
    }
}

#[get("/index/generate")]
async fn generate_index(state: &State) -> Json<String> {
    let index = build_index(&state.root_path);
    let index_str = serde_json::to_string(&index).unwrap();

    *state.index.write().await = Some(index);

    Json(index_str)
}

#[get("/stream/<id>")]
async fn stream(state: &State, id: u64) -> FaillibleResponse<Custom<File>> {
    let library = state.index.read().await;
    let library = library.as_ref().ok_or_else(|| {
        server_error(
            Status::NotFound,
            "Please generate a state first".to_string(),
        )
    })?;

    let file_info = library
        .tracks_files
        .get(&id)
        .ok_or_else(|| server_error(Status::NotFound, "Track ID was not found".to_string()))?;

    let file = File::open(Path::new(&file_info.path))
        .await
        .map_err(|err| {
            server_error(
                Status::InternalServerError,
                format!("Failed to open track file: {err}"),
            )
        })?;

    Ok(Custom(mime_type(file_info.format), file))
}

pub async fn launch(root_path: PathBuf) -> Result<(), rocket::Error> {
    rocket::build()
        .manage(ServerState {
            root_path,
            index: Arc::new(RwLock::new(None)),
        })
        .mount("/", routes![version, index, generate_index, stream])
        .launch()
        .await
}

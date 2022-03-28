use rocket::{http::Status, routes, State as RocketState};
use std::{path::PathBuf, sync::RwLock};

use crate::{builder::build_index, index::Library};

type State = RocketState<ServerState>;

pub struct ServerState {
    root_path: PathBuf,
    index: RwLock<Option<Library>>,
}

#[get("/version")]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[get("/index?<since>")]
pub fn index(state: &State, since: u64) -> (Status, String) {
    let index = state.index.read().unwrap();

    match *index {
        Some(ref index) => {
            if index.creation_time != since {
                (Status::Ok, serde_json::to_string(index).unwrap())
            } else {
                (Status::NotModified, "State didn't change".to_string())
            }
        }
        None => (
            Status::Accepted,
            "Please generate a state first".to_string(),
        ),
    }
}

#[get("/index/generate")]
pub fn generate_index(state: &State) -> String {
    let index = build_index(&state.root_path);
    let index_str = serde_json::to_string(&index).unwrap();

    *state.index.write().unwrap() = Some(index);

    index_str
}

pub async fn launch(root_path: PathBuf) -> Result<(), rocket::Error> {
    rocket::build()
        .manage(ServerState {
            root_path,
            index: RwLock::new(None),
        })
        .mount("/", routes![version, index, generate_index])
        .launch()
        .await
}

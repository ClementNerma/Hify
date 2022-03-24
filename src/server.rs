use std::{path::PathBuf, sync::RwLock};

use miniserde::json;
use rocket::{http::Status, routes, State as RocketState};

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

#[get("/state?<since>")]
pub fn state(state: &State, since: u64) -> (Status, String) {
    let index = state.index.read().unwrap();

    match *index {
        Some(ref index) => {
            if index.creation_time != since {
                (Status::Ok, json::to_string(index))
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

#[get("/state/generate")]
pub fn generate_state(state: &State) -> String {
    let index = build_index(&state.root_path);
    let index_str = json::to_string(&index);

    *state.index.write().unwrap() = Some(index);

    return index_str;
}

pub async fn launch(root_path: PathBuf) -> Result<(), rocket::Error> {
    rocket::build()
        .manage(ServerState {
            root_path,
            index: RwLock::new(None),
        })
        .mount("/", routes![version, state, generate_state])
        .launch()
        .await
}

#![forbid(unsafe_code)]
#![forbid(unused_must_use)]

use std::path::PathBuf;

mod builder;
mod ffprobe;
mod index;
mod server;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    let root_path = std::env::args().nth(1).expect("Please provide a root path");
    let root_path = PathBuf::from(root_path);

    if !root_path.is_dir() {
        panic!("Please provide a root directory");
    }

    server::launch(root_path).await.unwrap();
}

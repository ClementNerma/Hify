#![forbid(unsafe_code)]
#![forbid(unused_must_use)]

use clap::StructOpt;

mod cmd;
mod graphql;
mod http;
mod index;
mod userdata;
mod utils;

#[::rocket::main]
async fn main() {
    let cmd::Command {
        music_dir,
        index_file,
        user_data_file,
    } = cmd::Command::parse();

    if !music_dir.is_dir() {
        panic!("Music path is not a directory");
    }

    if index_file.is_dir() {
        panic!("Index file must not be a directory");
    }

    let index = if index_file.is_file() {
        println!("> Loading index from disk...");
        let index = utils::save::load_index(&index_file).unwrap();

        println!("> Done.");

        index
    } else {
        println!("> Generating index...");
        let index = index::build_index(music_dir);
        utils::save::save_index(&index_file, &index).unwrap();
        println!("> Index saved on disk.");

        index
    };

    let user_data = if user_data_file.is_file() {
        utils::save::load_user_data(&user_data_file).unwrap()
    } else {
        userdata::UserData::new(200)
    };

    let user_data = userdata::UserDataWrapper::new(
        user_data,
        Box::new(move |user_data| utils::save::save_user_data(&user_data_file, user_data).unwrap()),
    );

    println!("> Launching server...");

    let server = http::launch(index, user_data).await.unwrap();
    server.shutdown().await;
}

#![forbid(unsafe_code)]
#![forbid(unused_must_use)]

use clap::StructOpt;

mod cmd;
mod graphql;
mod index;
mod rocket;
mod utils;

#[::rocket::main]
async fn main() {
    let args = cmd::Command::parse();

    if !args.music_dir.is_dir() {
        panic!("Music path is not a directory");
    }

    let index = if args.index_file.is_dir() {
        panic!("Index file must not be a directory");
    } else if args.index_file.is_file() {
        println!("> Loading index from disk...");
        let index = utils::save::load_index(&args.index_file).unwrap();

        println!("> Done.");

        index
    } else {
        println!("> Generating index...");
        let index = index::build_index(args.music_dir);

        utils::save::save_index(&args.index_file, &index).unwrap();
        println!("> Index saved on disk.");

        index
    };

    println!("> Launching server...");

    rocket::launch(index).await.unwrap();
}

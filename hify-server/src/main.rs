#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
#![forbid(unused_crate_dependencies)]

mod cmd;
mod graphql;
mod http;
mod index;
mod library;
mod userdata;
mod utils;

use std::{fs, net::SocketAddr};

use anyhow::{bail, Context, Result};
use clap::Parser;

#[tokio::main]
async fn main() {
    if let Err(err) = inner_main().await {
        eprintln!("An error occurred:\n{err:?}");
    }
}

async fn inner_main() -> Result<()> {
    #[deny(unused_variables)]
    let cmd::Command {
        music_dir,
        data_dir,
        rebuild_index,
        update_index,
        rebuild_arts,
        rebuild_cache,
        no_server,
    } = cmd::Command::parse();

    if !music_dir.is_dir() {
        bail!("Music path is not a directory");
    }

    let data_dir = match data_dir {
        Some(data_dir) => data_dir,
        None => dirs::data_dir()
            .context("Failed to get path to the user's configuration directory")?
            .join("hify"),
    };

    if !data_dir.exists() {
        fs::create_dir(&data_dir).context("Failed to create the data directory")?;
    }

    let user_data_file = data_dir.join("userdata.json");
    let index_file = data_dir.join("index.json");

    let user_data = match user_data_file.is_file() {
        true => utils::save::load_user_data(&user_data_file).context("Failed to load user data")?,
        false => userdata::UserData::with_default_config(),
    };

    let mut user_data = userdata::UserDataWrapper::new(
        user_data,
        Box::new(move |user_data| {
            // TODO: error handling
            utils::save::save_user_data(&user_data_file, user_data).unwrap();
        }),
    );

    if index_file.is_dir() {
        bail!("Index file must not be a directory");
    }

    let index = match index_file.is_file() && !rebuild_index {
        true => {
            println!("> Loading index from disk...");
            let mut index = utils::save::load_index(&index_file).context("Failed to load index")?;

            if update_index {
                println!("> Updating index as requested...");

                index = index::build_index(music_dir, Some(index))
                    .context("Failed to rebuild index")?;

                utils::save::save_index(&index_file, &index)
                    .context("Failed to save index file with rebuilt cache")?;

                user_data.cleanup(&index);
            } else if rebuild_cache {
                println!("> Rebuilding cache as requested...");

                index::rebuild_cache(&mut index);

                utils::save::save_index(&index_file, &index)
                    .context("Failed to save index file with rebuilt cache")?;
            }

            if rebuild_arts {
                println!("> Rebuilding arts as requested...");

                index::rebuild_arts(&mut index);

                utils::save::save_index(&index_file, &index)
                    .context("Failed to save index file with rebuilt arts")?;
            }

            println!("> Done.");

            index
        }

        false => {
            println!("> Generating index...");
            let index = index::build_index(music_dir, None).context("Failed to build index")?;
            utils::save::save_index(&index_file, &index).context("Failed to save index file")?;
            println!("> Index saved on disk.");

            index
        }
    };

    if no_server {
        return Ok(());
    }

    println!("> Launching server...");

    // TODO: make it configurable
    let addr = "0.0.0.0:8893".parse::<SocketAddr>().unwrap();

    http::launch(
        &addr,
        index,
        user_data,
        Box::new(move |index| {
            utils::save::save_index(&index_file, index).map_err(|err| format!("{err:?}"))
        }),
    )
    .await
    .context("Failed to launch HTTP server")
}

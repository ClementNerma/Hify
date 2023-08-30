#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
#![warn(unused_crate_dependencies)]

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
use log::{error, info};

use crate::cmd::Args;

use self::utils::logging::setup_logger;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    setup_logger(args.logging_level, args.display_timestamps_in_tty);

    if let Err(err) = inner_main(args).await {
        error!("An error occurred:\n{err:?}");
    }
}

async fn inner_main(args: Args) -> Result<()> {
    #[deny(unused_variables)]
    let Args {
        music_dir,
        data_dir,
        rebuild_index,
        update_index,
        rebuild_arts,
        rebuild_cache,
        refetch_file_times,
        addr,
        port,
        no_server,

        // Options main() already took care of
        logging_level: _,
        display_timestamps_in_tty: _,
    } = args;

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
            info!("> Loading index from disk...");
            let mut index = utils::save::load_index(&index_file).context("Failed to load index")?;

            if update_index {
                info!("> Updating index as requested...");

                index = index::build_index(music_dir.clone(), Some(index))
                    .await
                    .context("Failed to rebuild index")?;

                utils::save::save_index(&index_file, &index)
                    .context("Failed to save index file with rebuilt cache")?;

                user_data.cleanup(&index);
            } else if refetch_file_times {
                info!("> Re-fetching file times...");
                index::refetch_file_times(&mut index)?;

                info!("> Rebuilding cache...");
                index::rebuild_cache(&mut index);

                utils::save::save_index(&index_file, &index)
                    .context("Failed to save index file with rebuilt cache")?;
            } else if rebuild_cache {
                info!("> Rebuilding cache as requested...");

                index::rebuild_cache(&mut index);

                utils::save::save_index(&index_file, &index)
                    .context("Failed to save index file with rebuilt cache")?;
            }

            if music_dir.exists() != index.from.exists()
                || fs::canonicalize(&music_dir)? != fs::canonicalize(&index.from)?
            {
                bail!(
                    "Provided music directory is {} but current index references {}",
                    music_dir.display(),
                    index.from.display()
                );
            }

            if rebuild_arts {
                info!("> Rebuilding arts as requested...");

                index::rebuild_arts(&mut index);

                utils::save::save_index(&index_file, &index)
                    .context("Failed to save index file with rebuilt arts")?;
            }

            info!("> Done.");

            index
        }

        false => {
            info!("> Generating index...");

            let index = index::build_index(music_dir.clone(), None)
                .await
                .context("Failed to build index")?;

            utils::save::save_index(&index_file, &index).context("Failed to save index file")?;
            info!("> Index saved on disk.");

            index
        }
    };

    if no_server {
        return Ok(());
    }

    info!("> Launching server...");

    http::launch(
        &SocketAddr::from((addr, port)),
        index,
        user_data,
        Box::new(move |index| {
            utils::save::save_index(&index_file, index).map_err(|err| format!("{err:?}"))
        }),
    )
    .await
    .context("Failed to launch HTTP server")
}

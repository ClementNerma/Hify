#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
#![warn(unused_crate_dependencies)]

mod check;
mod cmd;
mod graphql;
mod helpers;
mod http;
mod index;
mod library;
mod resources;
mod userdata;

use std::{fs, net::SocketAddr, process::ExitCode};

use anyhow::{bail, Context, Result};
use clap::Parser;
use log::{error, info};

use crate::{check::check_correctness, cmd::Args};

use self::{
    helpers::{logging::setup_logger, time::OFFSET},
    resources::ResourceManager,
};

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();

    // Trigger offset fetching
    let _ = *OFFSET;

    setup_logger(args.logging_level, args.display_timestamps_in_tty);

    if OFFSET.is_none() {
        error!("Failed to determine local offset, falling back to UTC.");
    }

    match inner_main(args).await {
        Ok(()) => ExitCode::SUCCESS,

        Err(err) => {
            error!("An error occurred:\n{err:?}");
            ExitCode::FAILURE
        }
    }
}

async fn inner_main(args: Args) -> Result<()> {
    #[deny(unused_variables)]
    let Args {
        music_dir,
        data_dir,
        rebuild_index,
        update_index,
        rebuild_resources,
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
            .context("Failed to get path to the user's data directory")?
            .join("hify"),
    };

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).context("Failed to create the data directory")?;
    }

    let generation_dir = dirs::state_dir()
        .context("Failed to get path to the user's state directory")?
        .join("hify")
        .join("generated");

    if !generation_dir.exists() {
        fs::create_dir_all(&generation_dir)
            .context("Failed to create the data generation directory")?;
    }

    let res_manager = ResourceManager::new(generation_dir);

    let user_data_file = data_dir.join("userdata.json");
    let index_file = data_dir.join("index.json");

    let user_data = match user_data_file.is_file() {
        true => {
            helpers::save::load_user_data(&user_data_file).context("Failed to load user data")?
        }
        false => userdata::UserDataInner::with_default_config(),
    };

    let mut user_data = userdata::UserData::new(
        user_data,
        Box::new(move |user_data| {
            // TODO: error handling
            helpers::save::save_user_data(&user_data_file, user_data).unwrap();
        }),
    );

    if index_file.is_dir() {
        bail!("Index file must not be a directory");
    }

    let index = match index_file.is_file() && !rebuild_index {
        true => {
            info!("> Loading index from disk...");
            let mut index =
                helpers::save::load_index(&index_file).context("Failed to load index")?;

            if update_index {
                info!("> Updating index as requested...");

                index = index::build_index(music_dir.clone(), Some(index), &res_manager)
                    .await
                    .context("Failed to rebuild index")?;

                helpers::save::save_index(&index_file, &index)
                    .context("Failed to save index file with rebuilt cache")?;

                user_data.cleanup(&index);
            } else if refetch_file_times {
                info!("> Re-fetching file times...");
                index::refetch_file_times(&mut index)?;

                info!("> Rebuilding cache...");
                index::rebuild_cache(&mut index);

                helpers::save::save_index(&index_file, &index)
                    .context("Failed to save index file with rebuilt cache")?;
            } else if rebuild_cache {
                info!("> Rebuilding cache as requested...");
                index::rebuild_cache(&mut index);

                helpers::save::save_index(&index_file, &index)
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

            if rebuild_resources {
                info!("> Rebuilding resources as requested...");

                index::rebuild_resources(&mut index, &res_manager).await?;

                helpers::save::save_index(&index_file, &index)
                    .context("Failed to save index file with rebuilt arts")?;
            }

            info!("> Done.");

            index
        }

        false => {
            info!("> Generating index...");

            let index = index::build_index(music_dir.clone(), None, &res_manager)
                .await
                .context("Failed to build index")?;

            helpers::save::save_index(&index_file, &index).context("Failed to save index file")?;
            info!("> Index saved on disk.");

            index
        }
    };

    info!("> Checking data correctness...");

    if let Err(errors) = check_correctness(&index, &user_data) {
        for err in &errors {
            error!("{err}\n");
        }

        bail!(
            "Correctness checking failed with {} error(s).",
            errors.len()
        );
    }

    if no_server {
        return Ok(());
    }

    info!("> Launching server...");

    http::launch(
        SocketAddr::from((addr, port)),
        index,
        user_data,
        res_manager,
        Box::new(move |index| {
            helpers::save::save_index(&index_file, index).map_err(|err| format!("{err:?}"))
        }),
    )
    .await
    .context("Failed to launch HTTP server")
}

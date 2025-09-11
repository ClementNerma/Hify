#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
#![warn(unused_crate_dependencies)]

mod check;
mod cmd;
mod graphql;
mod http;
mod index;
mod library;
mod logging;
mod resources;
mod userdata;

use std::{fs, net::SocketAddr, path::Path, process::ExitCode};

use anyhow::{Context, Result, bail, ensure};
use clap::Parser;
use log::{error, info};

use crate::{check::check_correctness, cmd::Args};

use self::{
    index::Index, logging::setup_logger, resources::ResourceManager, userdata::UserDataWrapper,
};

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();

    setup_logger(args.logging_level, args.display_timestamps_in_tty);

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

    ensure!(music_dir.is_dir(), "Music path is not a directory");

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

    let res_manager = ResourceManager::new(generation_dir)?;

    let user_data_file = data_dir.join("userdata.json");
    let index_file = data_dir.join("index.json");

    let mut user_data =
        UserDataWrapper::new_create(user_data_file.clone()).context("Failed to load user data")?;

    ensure!(!index_file.is_dir(), "Index file must not be a directory");

    let index = if index_file.is_file() && !rebuild_index {
        info!("> Loading index from disk...");

        let content = fs::read(&index_file)?;
        let json_str = std::str::from_utf8(&content)?;
        let mut index = serde_json::from_str(json_str).context("Failed to load index")?;

        if update_index {
            info!("> Updating index as requested...");

            index = index::build_index(music_dir.clone(), Some(index), &res_manager)
                .await
                .context("Failed to rebuild index")?;

            save_index(&index_file, &index)
                .context("Failed to save index file with rebuilt cache")?;

            user_data.cleanup(&index);
        } else if refetch_file_times {
            info!("> Re-fetching file times...");
            index::refetch_file_times(&mut index)?;

            info!("> Rebuilding cache...");
            index::rebuild_cache(&mut index);

            save_index(&index_file, &index)
                .context("Failed to save index file with rebuilt cache")?;
        } else if rebuild_cache {
            info!("> Rebuilding cache as requested...");
            index::rebuild_cache(&mut index);

            save_index(&index_file, &index)
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

            save_index(&index_file, &index)
                .context("Failed to save index file with rebuilt arts")?;
        }

        info!("> Done.");

        index
    } else {
        info!("> Generating index...");

        let index = index::build_index(music_dir.clone(), None, &res_manager)
            .await
            .context("Failed to build index")?;

        save_index(&index_file, &index).context("Failed to save index file")?;
        info!("> Index saved on disk.");

        index
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
        Box::new(move |index| save_index(&index_file, index).map_err(|err| format!("{err:?}"))),
    )
    .await
    .context("Failed to launch HTTP server")
}

pub fn save_index(to: &Path, index: &Index) -> Result<()> {
    let json = serde_json::to_string(index)?;
    fs::write(to, json)?;
    Ok(())
}

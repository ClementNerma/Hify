#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
#![warn(unused_crate_dependencies)]

mod arts;
mod changes;
mod check;
mod cmd;
mod graphql;
mod http;
mod index;
mod library;
mod logging;
mod resources;
mod runner;
mod userdata;

use std::{net::SocketAddr, process::ExitCode};

use anyhow::{Context, Result, bail, ensure};
use clap::Parser;
use log::{error, info};
use tokio::fs;

use crate::{check::check_correctness, cmd::Args};

use self::{
    changes::detect_changes, index::Index, logging::setup_logger, resources::ResourceManager,
    userdata::UserDataWrapper,
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
        None => dirs::state_dir()
            .context("Failed to get path to the user's state directory")?
            .join("hify"),
    };

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .await
            .context("Failed to create the data directory")?;
    }

    let music_dir_bis = music_dir.clone();

    let (res_manager, user_data, index) = tokio::task::spawn_blocking(
        move || -> Result<(ResourceManager, UserDataWrapper, Index)> {
            let res_manager = ResourceManager::load(data_dir)?;

            let mut user_data = res_manager.load_user_data()?;

            let index = if rebuild_index {
                info!("> Rebuilding tracks index as requested...");

                detect_changes(&music_dir_bis, &mut user_data, &res_manager, None)?
            } else {
                info!("> Loading tracks index from disk...");

                match res_manager.load_index()? {
                    Some(index) => {
                        if update_index {
                            info!("> Updating index as requested...");

                            detect_changes(
                                &music_dir_bis,
                                &mut user_data,
                                &res_manager,
                                Some(&index),
                            )?
                        } else {
                            index
                        }
                    }

                    None => {
                        info!("> Tracks index file not found, generating it...");

                        detect_changes(&music_dir_bis, &mut user_data, &res_manager, None)?
                    }
                }
            };

            Ok((res_manager, user_data, index))
        },
    )
    .await??;

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
        music_dir,
        index,
        user_data,
        res_manager,
    )
    .await
    .context("Failed to launch HTTP server")
}

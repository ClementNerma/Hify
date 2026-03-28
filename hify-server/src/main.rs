//
// Enable some strict rules
//
#![forbid(unsafe_code, unused_must_use)]
//
// Enable some additional warnings
//
#![warn(unused_crate_dependencies, missing_debug_implementations)]
//
// Enable all of Clippy's lints by default
//
#![warn(clippy::pedantic, clippy::cargo)]
//
// -> Enable some more lints from `restriction`
#![warn(clippy::as_conversions)]
//
// -> Then disable a few ones
//
#![allow(
    clippy::float_cmp,
    clippy::arithmetic_side_effects,
    clippy::integer_division,
    clippy::map_err_ignore,
    clippy::missing_const_for_fn,
    clippy::multiple_crate_versions,
    clippy::option_if_let_else,
    clippy::shadow_unrelated,
    clippy::unused_trait_names,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::wildcard_enum_match_arm,
    clippy::wildcard_imports,
    clippy::similar_names
)]

mod arts;
mod cmd;
mod index;
mod indexer;
mod logger;
mod manager;
mod server;
mod utils;

use std::process::ExitCode;

use anyhow::{Context, Result, bail};
use clap::Parser;
use log::error;
use tokio::{fs, task::spawn_blocking};

use self::{cmd::CmdArgs, logger::Logger, manager::DataManager};

#[tokio::main]
async fn main() -> ExitCode {
    let args = CmdArgs::parse();

    Logger::new(args.verbosity).init().unwrap();

    match inner_main(args).await {
        Ok(()) => ExitCode::SUCCESS,

        Err(err) => {
            error!("{err:?}");
            ExitCode::FAILURE
        }
    }
}

async fn inner_main(args: CmdArgs) -> Result<()> {
    let CmdArgs {
        music_dir,
        data_dir,
        verbosity: _,
        addr,
        port,
    } = args;

    if !fs::try_exists(&music_dir).await.is_ok_and(|b| b) {
        bail!(
            "Music directory '{}' is not a valid directory",
            music_dir.display()
        );
    }

    if !fs::try_exists(&data_dir).await.is_ok_and(|b| b) {
        fs::create_dir_all(&data_dir)
            .await
            .with_context(|| format!("Failed to create data directory '{}'", data_dir.display()))?;
    }

    let data_manager = spawn_blocking(move || DataManager::load(&data_dir, music_dir))
        .await
        .unwrap()?;

    server::launch((addr, port).into(), data_manager).await
}

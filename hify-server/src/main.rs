#![forbid(unsafe_code)]
#![forbid(unused_must_use)]

mod cmd;
mod graphql;
mod http;
mod index;
mod library;
mod userdata;
mod utils;

use anyhow::{bail, Context, Result};
use clap::StructOpt;

#[::rocket::main]
async fn main() {
    if let Err(err) = inner_main().await {
        eprintln!("An error occurred:\n{err:?}");
    }
}

async fn inner_main() -> Result<()> {
    #[deny(unused_variables)]
    let cmd::Command {
        music_dir,
        index_file,
        user_data_file,
        rebuild_index,
        update_index,
        rebuild_arts,
        rebuild_cache,
        no_server,
    } = cmd::Command::parse();

    if !music_dir.is_dir() {
        bail!("Music path is not a directory");
    }

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

    let user_data = match user_data_file.is_file() {
        true => utils::save::load_user_data(&user_data_file).context("Failed to load user data")?,
        false => userdata::UserData::with_default_config(),
    };

    let user_data = userdata::UserDataWrapper::new(
        user_data,
        Box::new(move |user_data| {
            // TODO: error handling
            utils::save::save_user_data(&user_data_file, user_data).unwrap();
        }),
    );

    if no_server {
        return Ok(());
    }

    println!("> Launching server...");

    let server = http::launch(
        index,
        user_data,
        Box::new(move |index| {
            utils::save::save_index(&index_file, index).map_err(|err| format!("{err:?}"))
        }),
    )
    .await
    .context("Failed to launch HTTP server")?;

    server.shutdown().await;

    Ok(())
}

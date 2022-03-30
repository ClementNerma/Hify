use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
    #[clap(short, long)]
    pub music_dir: PathBuf,

    #[clap(short, long)]
    pub index_file: PathBuf,
}

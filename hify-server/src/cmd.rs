use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
    #[clap(help = "Path to the directory containing the audio files")]
    pub music_dir: PathBuf,

    #[clap(short, long, help = "Path to the index file (JSON)")]
    pub index_file: PathBuf,

    #[clap(short, long, help = "Path to the user data file (JSON)")]
    pub user_data_file: PathBuf,

    #[clap(long, help = "Force to rebuild index from scratch")]
    pub rebuild_index: bool,

    #[clap(long, help = "Update the index")]
    pub update_index: bool,

    #[clap(long, help = "Rebuild the cache", conflicts_with = "update_index")]
    pub rebuild_cache: bool,

    #[clap(long, help = "Don't start the server")]
    pub no_server: bool,
}

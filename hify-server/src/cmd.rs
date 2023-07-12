use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
    #[clap(help = "Path to the directory containing the audio files")]
    pub music_dir: PathBuf,

    #[clap(short, long, help = "Path to the user data directory")]
    pub data_dir: Option<PathBuf>,

    #[clap(long, help = "Force to rebuild index from scratch")]
    pub rebuild_index: bool,

    #[clap(long, help = "Update the index")]
    pub update_index: bool,

    #[clap(
        long,
        help = "Force to rebuild arts index from scratch",
        conflicts_with = "rebuild_index"
    )]
    pub rebuild_arts: bool,

    #[clap(
        long,
        help = "Rebuild the cache",
        conflicts_with = "update_index",
        conflicts_with = "rebuild_index"
    )]
    pub rebuild_cache: bool,

    #[clap(
        long,
        help = "Fetch file times from filesystem",
        conflicts_with = "update_index",
        conflicts_with = "rebuild_index",
        conflicts_with = "rebuild_cache"
    )]
    pub refetch_file_times: bool,

    #[clap(long, help = "Don't start the server")]
    pub no_server: bool,
}

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
        conflicts_with = "rebuild-index"
    )]
    pub rebuild_arts: bool,

    #[clap(
        long,
        help = "Rebuild the cache",
        conflicts_with = "update-index",
        conflicts_with = "rebuild-index"
    )]
    pub rebuild_cache: bool,

    #[clap(long, help = "Don't start the server")]
    pub no_server: bool,
}

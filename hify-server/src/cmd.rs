use std::{net::IpAddr, path::PathBuf};

use clap::Parser;
use log::LevelFilter;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(help = "Path to the directory containing the audio files")]
    pub music_dir: PathBuf,

    #[clap(long, help = "Path to the user data directory")]
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

    #[clap(
        short,
        long,
        help = "Address to listen on (default: 0.0.0.0)",
        default_value = "0.0.0.0"
    )]
    pub addr: IpAddr,

    #[clap(
        short,
        long,
        help = "Port to listen on (default: 8893)",
        default_value = "8893"
    )]
    pub port: u16,

    #[clap(
        long,
        help = "Don't start the server",
        conflicts_with = "addr",
        conflicts_with = "port"
    )]
    pub no_server: bool,

    #[clap(short, long, help = "Logging level", default_value = "info")]
    pub logging_level: LevelFilter,

    #[clap(short, long, help = "Display timestamps in TTYs (disabled by default)")]
    pub display_timestamps_in_tty: bool,
}

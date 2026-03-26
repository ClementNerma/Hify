use std::{net::IpAddr, path::PathBuf};

use clap::Parser;
use log::LevelFilter;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct CmdArgs {
    #[clap(help = "Path to the music directory to index")]
    pub music_dir: PathBuf,

    #[clap(short, long, help = "Path to the data directory")]
    pub data_dir: PathBuf,

    #[clap(
        short,
        long,
        help = "Logging verbosity level (error, warn, info, debug, trace)",
        default_value = "info"
    )]
    pub verbosity: LevelFilter,

    #[clap(short, long, help = "Address to listen on", default_value = "0.0.0.0")]
    pub addr: IpAddr,

    #[clap(short, long, help = "Port to listen on", default_value = "8891")]
    pub port: u16,
}

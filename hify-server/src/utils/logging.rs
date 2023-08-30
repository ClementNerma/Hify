use std::io::IsTerminal;

use indicatif::{ProgressBar, ProgressStyle};
use log::LevelFilter;

use once_cell::sync::Lazy;

pub static IS_TERMINAL: Lazy<bool> = Lazy::new(|| std::io::stdout().is_terminal());

pub fn setup_logger(logging_level: LevelFilter, display_timestamps_in_tty: bool) {
    let mut builder = env_logger::builder();

    builder
        .filter_module("symphonia", LevelFilter::Warn)
        .format_target(false)
        .filter_level(logging_level);

    if *IS_TERMINAL && !display_timestamps_in_tty {
        builder.format_timestamp(None);
    }

    builder.init();
}

pub fn files_progress_bar(len: usize) -> ProgressBar {
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {eta_precise} {msg}")
        .expect("Invalid template provided for ProgressBar")
        .progress_chars("##-");

    let pb = ProgressBar::new(u64::try_from(len).unwrap()).with_style(style);

    // Display the progress bar immediatly
    pb.tick();

    pb
}

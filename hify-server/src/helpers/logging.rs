use std::{io::IsTerminal, sync::LazyLock, time::Duration};

use env_logger::TimestampPrecision;
use indicatif::{ProgressBar, ProgressStyle};
use log::LevelFilter;

pub static IS_TERMINAL: LazyLock<bool> = LazyLock::new(|| std::io::stdout().is_terminal());

pub fn setup_logger(logging_level: LevelFilter, display_timestamps_in_tty: bool) {
    env_logger::builder()
        // Hide informations from Symphonia
        .filter_module("symphonia", LevelFilter::Error)
        // Don't display module names
        .format_target(false)
        // Hide timestamp if requested to
        .format_timestamp(if *IS_TERMINAL && !display_timestamps_in_tty {
            None
        } else {
            Some(TimestampPrecision::default())
        })
        // Ignore logs with a level inferior to the requested one
        .filter_level(logging_level)
        // Initialize the logger
        .init();
}

pub fn progress_bar(len: usize) -> ProgressBar {
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {eta_precise} {msg}")
        .expect("Invalid template provided for ProgressBar")
        .progress_chars("##-");

    let pb = ProgressBar::new(u64::try_from(len).unwrap()).with_style(style);

    // Display the progress bar immediatly
    pb.tick();

    pb
}

pub fn spinner(template: &str) -> ProgressBar {
    let style = ProgressStyle::default_spinner()
        .template(&format!("{{spinner}} {template}"))
        .expect("Invalid template provided for ProgressBar")
        .tick_strings(&[
            "⢀⠀", "⡀⠀", "⠄⠀", "⢂⠀", "⡂⠀", "⠅⠀", "⢃⠀", "⡃⠀", "⠍⠀", "⢋⠀", "⡋⠀", "⠍⠁", "⢋⠁", "⡋⠁",
            "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩", "⠈⢙", "⠈⡙", "⢈⠩", "⡀⢙", "⠄⡙", "⢂⠩", "⡂⢘", "⠅⡘",
            "⢃⠨", "⡃⢐", "⠍⡐", "⢋⠠", "⡋⢀", "⠍⡁", "⢋⠁", "⡋⠁", "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩",
            "⠈⢙", "⠈⡙", "⠈⠩", "⠀⢙", "⠀⡙", "⠀⠩", "⠀⢘", "⠀⡘", "⠀⠨", "⠀⢐", "⠀⡐", "⠀⠠", "⠀⢀", "⠀⡀",
        ]);

    let spinner = ProgressBar::new_spinner().with_style(style);
    spinner.enable_steady_tick(Duration::from_millis(80));

    spinner
}

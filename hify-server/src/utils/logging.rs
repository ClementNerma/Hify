use std::io::IsTerminal;

use log::LevelFilter;

use once_cell::sync::Lazy;

pub static IS_TERMINAL: Lazy<bool> = Lazy::new(|| std::io::stdout().is_terminal());

pub fn setup_logger(logging_level: LevelFilter, display_timestamps_in_tty: bool) {
    let mut builder = env_logger::builder();

    builder.format_target(false).filter_level(logging_level);

    if *IS_TERMINAL && !display_timestamps_in_tty {
        builder.format_timestamp(None);
    }

    builder.init();
}

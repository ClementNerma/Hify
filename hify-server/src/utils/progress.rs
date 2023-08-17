use std::io::{stdout, Write};

use log::info;

use super::logging::IS_TERMINAL;

pub fn display_progress(elapsed: u64, current: usize, total: usize, errors: u64) {
    let minutes = elapsed / 60;
    let seconds = elapsed % 60;

    let message = format!(
        "Progress: {} / {} ({}%) in {}{}s...{}",
        current,
        total,
        current * 100 / total,
        if minutes > 0 {
            format!("{minutes}m ")
        } else {
            String::new()
        },
        seconds,
        if errors > 0 {
            format!(
                " (with {errors} error{})",
                if errors > 1 { "s" } else { "" }
            )
        } else {
            String::new()
        }
    );

    if *IS_TERMINAL {
        print!("\r        > {message}");
        stdout().flush().unwrap();
    } else {
        info!("{message}");
    }
}

pub fn clear_progress() {
    if *IS_TERMINAL {
        println!();
    }
}

use std::io::{stdout, Write};

pub fn display_progress(elapsed: u64, current: usize, total: usize, errors: u64) {
    let minutes = elapsed / 60;
    let seconds = elapsed % 60;

    print!(
        "\r        Progress: {} / {} ({}%) in {}{}s... {}",
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
            format!("(with {errors} error{})", if errors > 1 { "s" } else { "" })
        } else {
            String::new()
        }
    );

    stdout().flush().unwrap();
}

use std::io::{stdout, Write};

pub fn display_progress(elapsed: u64, current: usize, total: usize) {
    let minutes = elapsed / 60;
    let seconds = elapsed % 60;

    print!(
        "\r        Progress: {} / {} ({}%) in {}{}s... ",
        current,
        total,
        current * 100 / total,
        if minutes > 0 {
            format!("{minutes}m ")
        } else {
            String::new()
        },
        seconds
    );

    stdout().flush().unwrap();
}

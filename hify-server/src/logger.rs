use colored::Colorize;
use jiff::Zoned;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

pub struct Logger {
    level: LevelFilter,
}

impl Logger {
    pub fn new(level: LevelFilter) -> Self {
        Self { level }
    }

    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.level);
        log::set_boxed_logger(Box::new(self))
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level && metadata.target().starts_with(env!("CARGO_CRATE_NAME"))
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let msg = record.args().to_string();

        let colored = match record.level() {
            Level::Error => msg.bright_red(),
            Level::Warn => msg.bright_yellow(),
            Level::Info => msg.bright_blue(),
            Level::Debug => msg.bright_magenta(),
            Level::Trace => msg.bright_black(),
        };

        eprintln!(
            "{} {colored}",
            Zoned::now()
                .datetime()
                .strftime("[%Y-%m-%d %H:%M:%S]")
                .to_string()
                .bright_black()
        );
    }

    fn flush(&self) {}
}

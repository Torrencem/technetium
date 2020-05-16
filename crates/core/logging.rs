//! Application logging utilities for technetium
use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};

/// A very simple logger for the ``log`` crate
struct Logger {
    level: Level,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

/// Initialize the logging of the application to a given log::Level
pub fn init(level: Level) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(Logger { level }))
        .map(|()| log::set_max_level(LevelFilter::Trace))
}

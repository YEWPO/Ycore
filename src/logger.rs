use log::{Log, Level};

use crate::println;

pub struct SimpleLogger;

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("error") => log::LevelFilter::Error,
        Some("warn") => log::LevelFilter::Warn,
        Some("info") => log::LevelFilter::Info,
        Some("debug") => log::LevelFilter::Debug,
        Some("trace") => log::LevelFilter::Trace,
        _ => log::LevelFilter::Off,
    });
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        println!("\x1b[1;{}m{}\x1b[0m", level_color(record.level()), format_args!("[{:>5}] {}", record.level(), record.args()));
    }

    fn flush(&self) {
    }
}

fn level_color(level: Level) -> u8 {
    match level {
        Level::Error => 31u8,
        Level::Warn => 93u8,
        Level::Info => 34u8,
        Level::Debug => 32u8,
        Level::Trace => 90u8,
    }
}

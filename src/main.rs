pub mod args;
pub mod utils;

use args::{Command, LogArgs};
use clap::Parser;

pub mod command_handler;
use command_handler::{log_handler, open_handler, today_handler, undo_handler};

use log::{Level, LevelFilter, Metadata, Record};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} > {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

#[tokio::main]
async fn main() {
    log::set_logger(&CONSOLE_LOGGER).expect("Failed to launch logger");
    log::set_max_level(LevelFilter::Warn);

    let parsed_command = LogArgs::parse().command;
    let result = match parsed_command {
        Command::Log {
            time,
            task,
            message,
        } => log_handler(time, task, message).await,
        Command::Undo => undo_handler().await,
        Command::Open { task } => open_handler(task),
        Command::Today => today_handler().await,
    };

    if let Err(msg) = result {
        log::error!("{msg}");
    }
}

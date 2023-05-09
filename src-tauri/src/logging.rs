use std::{path::PathBuf, fs::{self, DirEntry}, cmp::Ordering};

use chrono::Local;
use log::{SetLoggerError, LevelFilter};
use log4rs::{append::{console::{ConsoleAppender, Target}, file::FileAppender}, encode::pattern::PatternEncoder, config::{Appender, Root}, filter::threshold::ThresholdFilter, Config};

const MAX_LOG_COUNT: usize = 10;

fn get_data_dir() -> PathBuf {
    dirs::data_dir().unwrap_or(PathBuf::from("/data"))
        .join("TableApp")
}

fn get_log_dir() -> PathBuf {
    get_data_dir().join("logs")
}

fn get_log_file() -> PathBuf {
    get_log_dir().join(format!("{}.log", Local::now().format("%Y-%m-%d %H-%M-%S")))
}

pub fn cleanup_logs() -> Result<(), std::io::Error> {
    if let Ok(dir) = fs::read_dir(get_log_dir()) {
        let mut entries = dir.filter_map(|e| match e {
            Ok(entry) => Some(entry),
            Err(_) => None,
        }).collect::<Vec<_>>();

        entries.sort_by(by_update_time);

        while entries.len() > MAX_LOG_COUNT {
            let entry = entries.pop().unwrap();
            fs::remove_file(entry.path())?;
        }
    }

    Ok(())
}

pub fn setup_logging() -> Result<(), SetLoggerError> {
    let level = log::LevelFilter::Info;
    let file_path = get_log_file();

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("[{d(%Y-%m-%d %H:%M:%S:%f)}][{l}] - {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?;

    Ok(())
}

fn by_update_time(a: &DirEntry, b: &DirEntry) -> Ordering {
    let a_metadata = match a.metadata() {
        Ok(metadata) => metadata,
        Err(_) => return Ordering::Equal
    };

    let b_metadata = match b.metadata() {
        Ok(metadata) => metadata,
        Err(_) => return Ordering::Equal
    };

    let a_modified = match a_metadata.modified() {
        Ok(modified) => modified,
        Err(_) => return Ordering::Equal
    };

    let b_modified = match b_metadata.modified() {
        Ok(modified) => modified,
        Err(_) => return Ordering::Equal
    };

    b_modified.cmp(&a_modified)
}
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::fs::OpenOptions;
use std::io::Write;

pub fn init_logger(log_level: Option<&str>, log_file: Option<&str>) {
    let log_level_filter = match log_level.unwrap_or("info") {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    let mut builder = Builder::new();
    builder.filter_level(log_level_filter);

    // Configure simple timestamp-based format
    builder.format(|buf, record| {
        writeln!(
            buf,
            "{} [{}] {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.args()
        )
    });

    match log_file {
        Some(file_path) if !file_path.is_empty() => {
            // If a non-empty log file is specified, add it as a target
            match OpenOptions::new().create(true).append(true).open(file_path) {
                Ok(file) => {
                    builder.target(Target::Pipe(Box::new(file)));
                    println!("Logging to file: {}", file_path);
                }
                Err(e) => {
                    eprintln!("Failed to open log file: {}", e);
                    // Fall back to console logging
                    builder.target(Target::Stdout);
                }
            }
        }
        _ => {
            // If log file is None, empty, or invalid, log to console
            builder.target(Target::Stdout);
        }
    }

    // Initialize the logger
    builder.init();
}

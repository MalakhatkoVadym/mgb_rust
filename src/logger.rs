use std::fs::OpenOptions;

use tracing::Level;

pub fn init_logger(verbose: bool, logger_file_path: String, log_level: Level) {
    if verbose {
        tracing_subscriber::fmt().with_max_level(log_level).init();
    } else {
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&logger_file_path)
            .unwrap_or_else(|_| {
                // If opening the specified file fails, attempt to open the default log file
                let default_path = crate::config::default_log_file_path();
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(default_path.clone())
                    .unwrap_or_else(|e| {
                        // If opening the default log file also fails, panic with an error message
                        panic!(
                            "Failed to open log file at '{}' and default path '{}': {}",
                            logger_file_path, default_path, e
                        )
                    })
            });

        tracing_subscriber::fmt()
            .with_max_level(log_level)
            .with_ansi(false)
            .with_writer(log_file)
            .init();
    }
}

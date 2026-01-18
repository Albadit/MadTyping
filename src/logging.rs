//! Logging utilities for MadTyping
//!
//! Provides simple file-based logging for debugging purposes.
//! Logging can be enabled/disabled via config::LOG_ENABLED.

use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Mutex,
    time::SystemTime,
};

use crate::config::LOG_ENABLED;

/// Global log file path
static LOG_FILE: Mutex<Option<PathBuf>> = Mutex::new(None);

/// Initialize the log file in the same directory as the executable.
/// Creates a new log file, overwriting any existing one.
pub fn init() {
    if !LOG_ENABLED {
        return;
    }
    
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let log_path = exe_dir.join("madtyping.log");
            // Clear old log
            let _ = fs::write(&log_path, "=== MadTyping Log Started ===\n");
            *LOG_FILE.lock().unwrap() = Some(log_path);
        }
    }
}

/// Write a message to the log file with a timestamp.
/// Does nothing if logging is disabled.
pub fn log(message: &str) {
    if !LOG_ENABLED {
        return;
    }
    
    let timestamp = timestamp();
    let log_line = format!("[{}] {}\n", timestamp, message);
    
    if let Some(path) = LOG_FILE.lock().unwrap().as_ref() {
        if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(path) {
            let _ = file.write_all(log_line.as_bytes());
        }
    }
}

/// Generate a simple HH:MM:SS timestamp without external crates.
fn timestamp() -> String {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs();
            let hours = (secs / 3600) % 24;
            let mins = (secs / 60) % 60;
            let s = secs % 60;
            format!("{:02}:{:02}:{:02}", hours, mins, s)
        }
        Err(_) => "??:??:??".to_string(),
    }
}

//! MadTyping - A League of Legends chat automation tool
//!
//! This library provides functionality to automatically type messages
//! into League of Legends chat by simulating keyboard input.
//!
//! # Architecture
//!
//! The crate is organized into the following modules:
//!
//! - [`config`] - Centralized configuration constants
//! - [`logging`] - Simple file-based logging utilities
//! - [`files`] - Text file discovery and management
//! - [`platform`] - Platform-specific input simulation (Windows)
//! - [`app`] - Application state management
//! - [`ui`] - Terminal UI rendering and event handling

pub mod config;
pub mod logging;
pub mod files;
pub mod platform;
pub mod app;
pub mod ui;

// Re-export commonly used items for convenience
pub use app::App;
pub use config::{DEFAULT_HEADER_NAME, DEFAULT_WINDOW_TITLE};
pub use files::{discover as discover_files, TextFile};
pub use logging::{init as init_logging, log};
pub use platform::{focus_window, is_window_running, send_text};
pub use ui::Cli;

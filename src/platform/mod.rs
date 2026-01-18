//! Platform-specific functionality
//!
//! This module provides cross-platform abstractions for window management
//! and keyboard input simulation.

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::*;

// Stub implementations for non-Windows platforms
#[cfg(not(windows))]
pub fn is_window_focused(_title: &str) -> bool {
    true
}

#[cfg(not(windows))]
pub fn is_window_running(_title: &str) -> bool {
    true
}

#[cfg(not(windows))]
pub fn focus_window(_title: &str) -> bool {
    true
}

#[cfg(not(windows))]
pub fn send_text(_text: &str, _window_title: &str) -> Result<(), String> {
    Err("Keyboard simulation only supported on Windows".to_string())
}

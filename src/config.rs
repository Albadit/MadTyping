//! Configuration constants for MadTyping
//!
//! This module centralizes all configurable delays and settings
//! for easy tuning and maintenance.

// ============== KEYBOARD INPUT DELAYS ==============

/// Delay between each character typed (lower = faster typing)
pub const CHAR_TYPE_DELAY_MS: u64 = 5;

/// Delay after focusing window before starting to type
pub const FOCUS_DELAY_MS: u64 = 50;

/// Delay after opening chat before typing message
pub const CHAT_OPEN_DELAY_MS: u64 = 100;

/// Delay after typing message before pressing Enter
pub const AFTER_TYPE_DELAY_MS: u64 = 30;

/// Delay after pressing Enter to send
pub const AFTER_SEND_DELAY_MS: u64 = 50;

/// Delay between key down and key up in a key press
pub const KEY_PRESS_DELAY_MS: u64 = 10;

/// Delay for Shift key operations
pub const SHIFT_KEY_DELAY_MS: u64 = 15;

/// Delay after SetForegroundWindow
pub const WINDOW_FOCUS_DELAY_MS: u64 = 100;

/// Delay for unicode character input
pub const UNICODE_KEY_DELAY_MS: u64 = 5;

// ============== CLI DELAYS ==============

/// Delay between sending each line of text
pub const NEXT_LINE_DELAY_MS: u64 = 100;

/// Delay for user to read messages (in seconds)
pub const USER_READ_DELAY_SECS: u64 = 2;

/// Delay after cancel before returning (in seconds)
pub const CANCEL_DELAY_SECS: u64 = 1;

// ============== LOGGING ==============

/// Set to false to disable logging
pub const LOG_ENABLED: bool = false;

// ============== APPLICATION ==============

/// Default application header name
pub const DEFAULT_HEADER_NAME: &str = "MadTyping - LoL Chat Tool";

/// Default target window title to search for
pub const DEFAULT_WINDOW_TITLE: &str = "League of Legends (TM) Client";

/// Supported file extensions for text files
pub const SUPPORTED_EXTENSIONS: &[&str] = &["txt", "md"];

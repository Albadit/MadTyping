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

/// Window title for the League of Legends client (lobby/launcher)
pub const CLIENT_WINDOW_TITLE: &str = "League of Legends";

/// Window title for the in-game League of Legends window
pub const GAME_WINDOW_TITLE: &str = "League of Legends (TM) Client";

/// A target window with its title and chat-open behavior.
#[derive(Clone, Debug)]
pub struct WindowTarget {
    /// The window title to search for
    pub title: &'static str,
    /// If true, use Shift+Enter to open chat (all-chat); if false, use Enter
    pub shift_enter: bool,
}

/// Default list of window targets to try, in priority order.
pub const WINDOW_TARGETS: &[WindowTarget] = &[
    WindowTarget {
        title: CLIENT_WINDOW_TITLE,
        shift_enter: false,
    },
    WindowTarget {
        title: GAME_WINDOW_TITLE,
        shift_enter: true,
    },
];

/// Replace spaces with ░ in file lines to preserve ASCII art alignment in chat.
/// Set to false to send lines with normal spaces.
pub const REPLACE_SPACES: bool = true;

/// The character to replace spaces with when REPLACE_SPACES is enabled.
pub const SPACE_REPLACEMENT: char = '░';

/// Supported file extensions for text files
pub const SUPPORTED_EXTENSIONS: &[&str] = &["txt", "md"];

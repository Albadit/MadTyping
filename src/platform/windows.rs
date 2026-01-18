//! Windows-specific input simulation and window management
//!
//! Provides low-level keyboard input simulation using Windows SendInput API
//! and window management using Win32 APIs.

use std::{
    sync::{Mutex, OnceLock},
    thread,
    time::Duration,
};

use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowTextW, EnumWindows, 
    SetForegroundWindow, ShowWindow, SW_RESTORE, SW_SHOW,
};
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::core::BOOL;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, 
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, KEYEVENTF_UNICODE,
    VK_RETURN, VK_SHIFT, MapVirtualKeyW, MAPVK_VK_TO_VSC, VkKeyScanW,
};

use crate::config::{
    CHAR_TYPE_DELAY_MS, FOCUS_DELAY_MS, CHAT_OPEN_DELAY_MS,
    AFTER_TYPE_DELAY_MS, AFTER_SEND_DELAY_MS, KEY_PRESS_DELAY_MS,
    SHIFT_KEY_DELAY_MS, WINDOW_FOCUS_DELAY_MS, UNICODE_KEY_DELAY_MS,
};
use crate::logging::log;

// ============== Window Management ==============

/// Check if a window with the given title is currently focused.
pub fn is_window_focused(target_title: &str) -> bool {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.0.is_null() {
            return false;
        }

        let mut buffer = [0u16; 256];
        let len = GetWindowTextW(hwnd, &mut buffer);
        
        if len == 0 {
            return false;
        }

        let title = String::from_utf16_lossy(&buffer[..len as usize]);
        title.to_lowercase().contains(&target_title.to_lowercase())
    }
}

/// Check if a window with the given title exists (without focusing it).
pub fn is_window_running(target_title: &str) -> bool {
    static CHECK_FOUND: OnceLock<Mutex<bool>> = OnceLock::new();
    static CHECK_TERM: OnceLock<Mutex<String>> = OnceLock::new();
    
    let found = CHECK_FOUND.get_or_init(|| Mutex::new(false));
    let search_term = CHECK_TERM.get_or_init(|| Mutex::new(String::new()));
    
    {
        let mut term = search_term.lock().unwrap();
        *term = target_title.to_lowercase();
        let mut f = found.lock().unwrap();
        *f = false;
    }
    
    unsafe extern "system" fn check_callback(hwnd: HWND, _: LPARAM) -> BOOL {
        let found = CHECK_FOUND.get().unwrap();
        let search_term = CHECK_TERM.get().unwrap();
        
        let mut buffer = [0u16; 256];
        let len = GetWindowTextW(hwnd, &mut buffer);
        
        if len > 0 {
            let title = String::from_utf16_lossy(&buffer[..len as usize]);
            let term = search_term.lock().unwrap();
            if title.to_lowercase().contains(term.as_str()) {
                let mut f = found.lock().unwrap();
                *f = true;
                return BOOL(0);
            }
        }
        BOOL(1)
    }
    
    unsafe {
        let _ = EnumWindows(Some(check_callback), LPARAM(0));
    }
    
    *found.lock().unwrap()
}

/// Find and focus a window by title (case-insensitive partial match).
pub fn focus_window(target_title: &str) -> bool {
    static FOUND_HWND: OnceLock<Mutex<Option<isize>>> = OnceLock::new();
    static SEARCH_TERM: OnceLock<Mutex<String>> = OnceLock::new();
    
    log(&format!("focus_window() called with target: '{}'", target_title));
    
    let found_hwnd = FOUND_HWND.get_or_init(|| Mutex::new(None));
    let search_term = SEARCH_TERM.get_or_init(|| Mutex::new(String::new()));
    
    // Set the search term
    {
        let mut term = search_term.lock().unwrap();
        *term = target_title.to_lowercase();
    }
    
    unsafe extern "system" fn enum_callback(hwnd: HWND, _: LPARAM) -> BOOL {
        let found_hwnd = FOUND_HWND.get().unwrap();
        let search_term = SEARCH_TERM.get().unwrap();
        
        let mut buffer = [0u16; 256];
        let len = GetWindowTextW(hwnd, &mut buffer);
        
        if len > 0 {
            let title = String::from_utf16_lossy(&buffer[..len as usize]);
            let term = search_term.lock().unwrap();
            if title.to_lowercase().contains(term.as_str()) {
                log(&format!("  Found matching window: '{}'", title));
                let mut found = found_hwnd.lock().unwrap();
                *found = Some(hwnd.0 as isize);
                return BOOL(0); // Stop enumeration
            }
        }
        BOOL(1) // Continue enumeration
    }
    
    // Reset found handle
    {
        let mut found = found_hwnd.lock().unwrap();
        *found = None;
    }
    
    unsafe {
        log("  Enumerating windows...");
        let _ = EnumWindows(Some(enum_callback), LPARAM(0));
        
        let found = found_hwnd.lock().unwrap();
        if let Some(hwnd_val) = *found {
            let hwnd = HWND(hwnd_val as *mut std::ffi::c_void);
            log("  Calling ShowWindow(SW_RESTORE)...");
            let _ = ShowWindow(hwnd, SW_RESTORE);
            log("  Calling ShowWindow(SW_SHOW)...");
            let _ = ShowWindow(hwnd, SW_SHOW);
            log("  Calling SetForegroundWindow...");
            let _ = SetForegroundWindow(hwnd);
            thread::sleep(Duration::from_millis(WINDOW_FOCUS_DELAY_MS));
            log("  Window focused successfully!");
            return true;
        }
    }
    
    log("  ERROR: Window not found!");
    false
}

// ============== Keyboard Input ==============

/// Send a key down event using scan codes.
fn send_key_down(vk: u16) {
    unsafe {
        let scan = MapVirtualKeyW(vk as u32, MAPVK_VK_TO_VSC) as u16;
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY(vk),
                    wScan: scan,
                    dwFlags: KEYEVENTF_SCANCODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

/// Send a key up event using scan codes.
fn send_key_up(vk: u16) {
    unsafe {
        let scan = MapVirtualKeyW(vk as u32, MAPVK_VK_TO_VSC) as u16;
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY(vk),
                    wScan: scan,
                    dwFlags: KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

/// Send a complete key press (down + delay + up).
fn send_key_press(vk: u16) {
    send_key_down(vk);
    thread::sleep(Duration::from_millis(KEY_PRESS_DELAY_MS));
    send_key_up(vk);
}

/// Send a single character, handling shift and unicode as needed.
fn send_char(c: char) {
    unsafe {
        // Try using VkKeyScan for ASCII characters
        let vk_result = VkKeyScanW(c as u16);
        
        if vk_result != -1 {
            let vk = (vk_result & 0xFF) as u16;
            let shift = (vk_result >> 8) & 1 != 0;
            
            if shift {
                send_key_down(VK_SHIFT.0);
                thread::sleep(Duration::from_millis(SHIFT_KEY_DELAY_MS));
            }
            
            send_key_press(vk);
            
            if shift {
                thread::sleep(Duration::from_millis(SHIFT_KEY_DELAY_MS));
                send_key_up(VK_SHIFT.0);
            }
        } else {
            // Use Unicode input for special characters
            let input_down = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY(0),
                        wScan: c as u16,
                        dwFlags: KEYEVENTF_UNICODE,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            let input_up = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY(0),
                        wScan: c as u16,
                        dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            SendInput(&[input_down], std::mem::size_of::<INPUT>() as i32);
            thread::sleep(Duration::from_millis(UNICODE_KEY_DELAY_MS));
            SendInput(&[input_up], std::mem::size_of::<INPUT>() as i32);
        }
    }
    thread::sleep(Duration::from_millis(CHAR_TYPE_DELAY_MS));
}

/// Type a string character by character.
fn type_text(text: &str) {
    for c in text.chars() {
        send_char(c);
    }
}

/// Send text to the target application.
///
/// This function:
/// 1. Checks if the target window is running
/// 2. Focuses the target window
/// 3. Opens all-chat with Shift+Enter
/// 4. Types the message
/// 5. Sends with Enter
pub fn send_text(text: &str, window_title: &str) -> Result<(), String> {
    let preview: String = text.chars().take(30).collect();
    log(&format!("send_text() called with: '{}'", preview));
    
    // First check if the window is running
    log(&format!("Checking if '{}' is running...", window_title));
    if !is_window_running(window_title) {
        log("ERROR: Application is not running!");
        return Err(format!("'{}' is not running. Please start the application first.", window_title));
    }
    log("Application is running, proceeding to focus...");
    
    // Focus target window before sending
    if !focus_window(window_title) {
        log("ERROR: Failed to focus window");
        return Err(format!("Window '{}' not found.", window_title));
    }

    // Wait for window to be fully focused
    thread::sleep(Duration::from_millis(FOCUS_DELAY_MS));

    // Step 1: Shift+Enter to open all chat
    log("Step 1: Pressing Shift+Enter to open chat...");
    send_key_down(VK_SHIFT.0);
    thread::sleep(Duration::from_millis(SHIFT_KEY_DELAY_MS));
    send_key_press(VK_RETURN.0);
    thread::sleep(Duration::from_millis(SHIFT_KEY_DELAY_MS));
    send_key_up(VK_SHIFT.0);
    log("  Shift+Enter sent");
    
    // Wait for chat to open
    thread::sleep(Duration::from_millis(CHAT_OPEN_DELAY_MS));

    // Step 2: Type the message character by character
    log(&format!("Step 2: Typing message ({} chars)...", text.len()));
    type_text(text);
    log("  Text typed successfully");
    
    // Wait for text to be fully typed
    thread::sleep(Duration::from_millis(AFTER_TYPE_DELAY_MS));

    // Step 3: Enter to send the message
    log("Step 3: Pressing Enter to send...");
    send_key_press(VK_RETURN.0);
    log("  Enter pressed");

    // Wait before next message
    thread::sleep(Duration::from_millis(AFTER_SEND_DELAY_MS));
    log("send_text() completed successfully");

    Ok(())
}

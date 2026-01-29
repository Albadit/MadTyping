# MadTyping 🎮⌨️

A Windows command-line tool for automating League of Legends chat messages. MadTyping reads text from `.txt` and `.md` files and automatically types them into the game chat, perfect for copy-pastas, pre-written messages, or any text you want to quickly send in-game.

## Features

- 📁 **Automatic File Discovery** - Scans for `.txt` and `.md` files in the executable directory
- 🔍 **Real-time Search** - Filter files by name as you type
- 👁️ **File Preview** - View file contents before sending
- ⚡ **Fast Keyboard Simulation** - Uses Windows SendInput API for reliable typing
- 🎯 **Smart Window Detection** - Automatically finds and focuses the League client
- 🔄 **Refresh Support** - Hot-reload files without restarting
- ⏹️ **Cancel Anytime** - Press ESC to stop sending mid-message

## Requirements

- **Windows** (uses Win32 APIs for keyboard simulation)
- **Rust** (for building from source)
- League of Legends client running

## Installation

### From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/albadit/MadTyping.git
   cd MadTyping
   ```

2. Build with Cargo:
   ```bash
   cargo build --release
   ```

3. The executable will be at `target/release/mad_typing.exe`

## Usage

1. **Place text files** in the same directory as `mad_typing.exe`
   - Supported formats: `.txt`, `.md`
   - Each non-empty line becomes a separate chat message

2. **Run the application**:
   ```bash
   mad_typing.exe
   ```

3. **Navigate the interface**:
   | Key | Action |
   |-----|--------|
   | `↑` `↓` | Navigate file list |
   | `Enter` | Send file contents to LoL chat |
   | `Tab` | Preview file contents |
   | `F5` | Refresh file list |
   | `Esc` | Quit / Cancel sending |
   | `Type` | Filter files by name |
   | `Backspace` | Remove search characters |

4. **Sending Messages**:
   - Select a file and press Enter
   - The tool will focus the League client window
   - Each line is typed and sent automatically
   - Press ESC during sending to cancel

## Example Text File

Create a file called `gg.txt`:
```
Good game everyone!
Well played!
```

When sent, each line will be typed as a separate chat message.

## Configuration

Key timing delays can be adjusted in [src/config.rs](src/config.rs):

```rust
// Keyboard input delays (in milliseconds)
pub const CHAR_TYPE_DELAY_MS: u64 = 5;      // Delay between characters
pub const NEXT_LINE_DELAY_MS: u64 = 100;    // Delay between messages
pub const FOCUS_DELAY_MS: u64 = 50;         // Delay after focusing window

// Application settings
pub const DEFAULT_WINDOW_TITLE: &str = "League of Legends (TM) Client";
```

## Project Structure

```
src/
├── main.rs      # Entry point
├── lib.rs       # Library exports
├── app.rs       # Application state management
├── config.rs    # Configuration constants
├── files.rs     # File discovery and loading
├── logging.rs   # Debug logging utilities
├── ui.rs        # Terminal UI rendering
└── platform/
    ├── mod.rs
    └── windows.rs  # Windows API integration
```

## Dependencies

- [crossterm](https://crates.io/crates/crossterm) - Cross-platform terminal manipulation
- [windows](https://crates.io/crates/windows) - Windows API bindings

## How It Works

1. The tool scans for text files in its directory
2. Displays an interactive TUI for file selection
3. When you select a file:
   - Finds the League of Legends window
   - Focuses it using `SetForegroundWindow`
   - For each line in the file:
     - Opens chat (presses Enter)
     - Types the message character-by-character
     - Sends the message (presses Enter)

## Troubleshooting

**No files found?**
- Make sure `.txt` or `.md` files are in the same folder as the executable
- Files must contain at least one non-empty line

**Messages not typing?**
- Ensure League of Legends is running
- Run the tool as Administrator if window focus issues occur
- Check that the window title matches (default: "League of Legends (TM) Client")

**Typing too fast/slow?**
- Adjust the delay constants in `config.rs` and rebuild

## License

This project is provided as-is for educational purposes.

## Disclaimer

⚠️ **Use responsibly.** This tool simulates keyboard input. Be mindful of:
- Game chat policies and terms of service
- Other players' experience
- Spam and harassment guidelines

The author is not responsible for any consequences of using this tool.

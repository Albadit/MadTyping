//! Terminal UI rendering and event handling for MadTyping
//!
//! Provides the interactive terminal interface including:
//! - File list display with search filtering
//! - File content viewer
//! - Message sending progress display

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind, poll, read},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use crate::app::App;
use crate::config::{NEXT_LINE_DELAY_MS, USER_READ_DELAY_SECS, CANCEL_DELAY_SECS};
use crate::logging::log;
use crate::platform::{is_window_running, send_text};

/// CLI renderer and event handler.
/// 
/// Manages terminal rendering and user input for the main interface
/// and file viewer.
pub struct Cli {
    stdout: io::Stdout,
    header_name: String,
    window_title: String,
}

impl Cli {
    /// Create a new CLI instance with custom header and target window title.
    pub fn new(header_name: String, window_title: String) -> Self {
        Self {
            stdout: io::stdout(),
            header_name,
            window_title,
        }
    }

    /// Initialize the terminal for the interactive UI.
    pub fn init(&mut self) -> Result<(), String> {
        terminal::enable_raw_mode()
            .map_err(|e| format!("Failed to enable raw mode: {}", e))?;
        
        execute!(self.stdout, EnterAlternateScreen, Hide)
            .map_err(|e| format!("Failed to setup terminal: {}", e))?;
        
        Ok(())
    }

    /// Cleanup the terminal state.
    pub fn cleanup(&mut self) -> Result<(), String> {
        execute!(self.stdout, LeaveAlternateScreen, Show)
            .map_err(|e| format!("Failed to cleanup terminal: {}", e))?;
        terminal::disable_raw_mode()
            .map_err(|e| format!("Failed to disable raw mode: {}", e))?;
        Ok(())
    }

    /// Render static header (only needs to be called once or on full refresh).
    fn render_header(&mut self) -> io::Result<()> {
        execute!(self.stdout, MoveTo(0, 0))?;
        
        let header_line = format!("  {}  ", self.header_name);
        let padding = (63 - header_line.len()) / 2;
        let header_centered = format!(
            "{}{}{}",
            " ".repeat(padding),
            header_line,
            " ".repeat(63 - padding - header_line.len())
        );
        
        execute!(
            self.stdout,
            SetForegroundColor(Color::Cyan),
            Print("═══════════════════════════════════════════════════════════════\n"),
            Print(format!("{}\n", header_centered)),
            Print("═══════════════════════════════════════════════════════════════\n"),
            ResetColor
        )?;
        Ok(())
    }

    /// Render static footer (only needs to be called once or on full refresh).
    fn render_footer(&mut self) -> io::Result<()> {
        let (_, height) = terminal::size().unwrap_or((80, 24));
        let footer_y = height.saturating_sub(3);
        execute!(self.stdout, MoveTo(0, footer_y))?;
        
        execute!(
            self.stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("───────────────────────────────────────────────────────────────\n"),
            ResetColor,
            SetForegroundColor(Color::Green),
            Print(" [↑↓] Navigate │ [Enter] Run │ [Tab] View │ [F5] Refresh │ [Esc] Quit"),
            ResetColor
        )?;
        Ok(())
    }

    /// Render the dynamic content area (search box, file list, error message).
    fn render_content(&mut self, app: &App) -> io::Result<()> {
        let (_, height) = terminal::size().unwrap_or((80, 24));
        
        // Search box (line 4)
        execute!(self.stdout, MoveTo(0, 4))?;
        execute!(
            self.stdout,
            Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::Yellow),
            Print(" Search: "),
            ResetColor,
            SetForegroundColor(Color::White),
            Print(app.search_query()),
            SetForegroundColor(Color::DarkGrey),
            Print("█"),
            ResetColor,
            Print(format!("  ({} files)  ", app.filtered_count())),
        )?;

        let visible_files = (height as usize).saturating_sub(12);
        let filtered = app.filtered_files();
        
        let scroll_offset = if app.selected_index() >= visible_files {
            app.selected_index() - visible_files + 1
        } else {
            0
        };

        // Clear file list area and display files
        let file_start_y = 6;
        for row in 0..visible_files {
            execute!(
                self.stdout,
                MoveTo(0, (file_start_y + row) as u16),
                Clear(ClearType::CurrentLine)
            )?;
        }

        // Display filtered files
        if filtered.is_empty() {
            execute!(
                self.stdout,
                MoveTo(0, file_start_y as u16),
                SetForegroundColor(Color::DarkGrey),
                Print("   No files match your search."),
                ResetColor
            )?;
        } else {
            for (i, file) in filtered.iter().enumerate().skip(scroll_offset).take(visible_files) {
                execute!(self.stdout, MoveTo(0, (file_start_y + i - scroll_offset) as u16))?;

                if i == app.selected_index() {
                    execute!(
                        self.stdout,
                        SetBackgroundColor(Color::DarkBlue),
                        SetForegroundColor(Color::White),
                        Print(format!(" ► {} ", file.name)),
                        ResetColor,
                        SetForegroundColor(Color::DarkGrey),
                        Print(format!("  ({} lines)", file.lines.len())),
                        ResetColor
                    )?;
                } else {
                    execute!(
                        self.stdout,
                        Print(format!("   {} ", file.name)),
                        SetForegroundColor(Color::DarkGrey),
                        Print(format!("  ({} lines)", file.lines.len())),
                        ResetColor
                    )?;
                }
            }
        }

        // Error message area (just above footer)
        let error_y = height.saturating_sub(5);
        execute!(
            self.stdout,
            MoveTo(0, error_y),
            Clear(ClearType::CurrentLine)
        )?;
        
        if let Some(error) = app.get_error() {
            execute!(
                self.stdout,
                SetForegroundColor(Color::Red),
                Print(format!(" ⚠ {} ", error)),
                ResetColor
            )?;
        }

        self.stdout.flush()?;
        Ok(())
    }

    /// Full render - clears screen and renders everything (header, content, footer).
    pub fn render(&mut self, app: &App) -> io::Result<()> {
        execute!(self.stdout, Clear(ClearType::All))?;
        self.render_header()?;
        self.render_content(app)?;
        self.render_footer()?;
        self.stdout.flush()?;
        Ok(())
    }

    /// Run the main event loop.
    pub fn run(&mut self, app: &mut App) -> Result<(), String> {
        // Initial full render (header + content + footer)
        if let Err(e) = self.render(app) {
            return Err(format!("Render error: {}", e));
        }

        loop {
            // Wait for input (blocking until event occurs)
            if let Ok(Event::Key(key_event)) = event::read() {
                // Only handle key press events, ignore release events
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }
                
                // Track if we need full render (header/footer changed or screen was cleared)
                let mut needs_full_render = false;
                
                match key_event.code {
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    KeyCode::F(5) => {
                        // Refresh file list - needs full render
                        match app.refresh_files() {
                            Ok(_) => {
                                log("File list refreshed");
                            }
                            Err(e) => {
                                app.set_error(format!("Refresh failed: {}", e));
                            }
                        }
                        needs_full_render = true;
                    }
                    KeyCode::Tab => {
                        // View file contents
                        if let Some(file) = app.get_selected() {
                            let lines = file.lines.clone();
                            let file_name = file.name.clone();
                            self.view_file(&file_name, &lines)?;
                        }
                        // After returning from view, need full render
                        needs_full_render = true;
                    }
                    KeyCode::Up => {
                        app.clear_error();
                        app.move_up();
                    }
                    KeyCode::Down => {
                        app.clear_error();
                        app.move_down();
                    }
                    KeyCode::Backspace => {
                        if !app.is_search_empty() {
                            app.clear_error();
                            app.remove_search_char();
                        } else {
                            continue; // Don't re-render if nothing to delete
                        }
                    }
                    KeyCode::Char(c) => {
                        app.clear_error();
                        app.add_search_char(c);
                    }
                    KeyCode::Enter => {
                        app.clear_error(); // Clear any previous error first
                        
                        if let Some(file) = app.get_selected() {
                            let lines = file.lines.clone();
                            let file_name = file.name.clone();
                            
                            log(&format!("User selected file: '{}' with {} lines", file_name, lines.len()));
                            
                            // Check if target window is running before proceeding
                            if !is_window_running(&self.window_title) {
                                log("ERROR: Target window is not running!");
                                app.set_error(format!("'{}' is not running!", self.window_title));
                            } else {
                                // Exit CLI to send messages (send_text will handle window focus)
                                self.cleanup()?;
                                
                                // Clear screen before showing progress
                                print!("\x1B[2J\x1B[1;1H");
                                
                                println!(">>> Selected: {}", file_name);
                                println!(">>> Sending {} lines...\n", lines.len());

                                self.send_all_lines(&lines);
                                
                                log("All messages sent, re-initializing CLI...");
                                // Re-initialize CLI and continue
                                self.init()?;
                                needs_full_render = true;
                            }
                        }
                    }
                    _ => continue, // Don't re-render for unhandled keys
                }
                
                // Re-render after handling input
                let render_result = if needs_full_render {
                    self.render(app) // Full render with header/footer
                } else {
                    self.render_content(app) // Only update content area
                };
                
                if let Err(e) = render_result {
                    return Err(format!("Render error: {}", e));
                }
            }
        }
    }

    /// Send all lines from the selected file (with cancel support).
    fn send_all_lines(&self, lines: &[String]) {
        let total = lines.len();
        
        println!("Press [Esc] to cancel at any time.\n");
        
        for (i, line) in lines.iter().enumerate() {
            // Check for Esc key to cancel
            if poll(Duration::from_millis(10)).unwrap_or(false) {
                if let Ok(Event::Key(key)) = read() {
                    if key.code == KeyCode::Esc {
                        println!("\n⚠ Cancelled by user.");
                        println!("\nReturning to file selection...");
                        thread::sleep(Duration::from_secs(CANCEL_DELAY_SECS));
                        return;
                    }
                }
            }
            
            // Calculate width for consistent formatting
            let width = total.to_string().len();
            println!(
                "[{:>width$}/{:>width$}] Sending: {}",
                i + 1,
                total,
                truncate_line(line, 50),
                width = width
            );

            match send_text(line, &self.window_title) {
                Ok(()) => {
                    thread::sleep(Duration::from_millis(NEXT_LINE_DELAY_MS));
                }
                Err(e) => {
                    println!("❌ Error: {}", e);
                    println!("Stopping. Make sure the target window is open.");
                    thread::sleep(Duration::from_secs(USER_READ_DELAY_SECS));
                    return;
                }
            }
        }

        println!("\n✅ Done! Sent {} messages.", total);
        println!("\nReturning to file selection...");
        thread::sleep(Duration::from_secs(USER_READ_DELAY_SECS));
    }

    /// View file contents in a scrollable viewer.
    fn view_file(&mut self, file_name: &str, lines: &[String]) -> Result<(), String> {
        let mut scroll_offset: usize = 0;
        let mut last_scroll_offset: usize = usize::MAX; // Force initial render
        let (_, term_height) = terminal::size().unwrap_or((80, 24));
        let visible_lines = (term_height as usize).saturating_sub(6);
        
        // Initial full render with header
        execute!(
            self.stdout,
            Clear(ClearType::All),
            MoveTo(0, 0)
        ).map_err(|e| e.to_string())?;
        
        // Static header (only rendered once)
        execute!(
            self.stdout,
            SetForegroundColor(Color::Cyan),
            Print("═══════════════════════════════════════════════════════════════\n"),
            Print(format!("                   Viewing: {}\n", file_name)),
            Print("═══════════════════════════════════════════════════════════════\n"),
            ResetColor
        ).map_err(|e| e.to_string())?;
        
        // Static footer separator (only rendered once)
        let footer_y = term_height.saturating_sub(2);
        execute!(
            self.stdout,
            MoveTo(0, footer_y),
            SetForegroundColor(Color::DarkGrey),
            Print("───────────────────────────────────────────────────────────────"),
            ResetColor
        ).map_err(|e| e.to_string())?;
        
        loop {
            // Only render content if scroll position changed
            if scroll_offset != last_scroll_offset {
                last_scroll_offset = scroll_offset;
                
                // Render content area only
                let content_start_y = 4;
                let end = (scroll_offset + visible_lines).min(lines.len());
                
                // Clear and render content lines
                for row in 0..visible_lines {
                    execute!(
                        self.stdout,
                        MoveTo(0, (content_start_y + row) as u16),
                        Clear(ClearType::CurrentLine)
                    ).map_err(|e| e.to_string())?;
                    
                    let line_idx = scroll_offset + row;
                    if line_idx < lines.len() {
                        let line_num = line_idx + 1;
                        execute!(
                            self.stdout,
                            SetForegroundColor(Color::DarkGrey),
                            Print(format!("{:4} │ ", line_num)),
                            ResetColor,
                            Print(&lines[line_idx])
                        ).map_err(|e| e.to_string())?;
                    }
                }
                
                // Update footer info line (dynamic scroll info)
                let scroll_info = format!("Lines {}-{} of {}", scroll_offset + 1, end, lines.len());
                execute!(
                    self.stdout,
                    MoveTo(0, footer_y + 1),
                    Clear(ClearType::CurrentLine),
                    SetForegroundColor(Color::Green),
                    Print(format!(" [↑↓] Scroll │ [Esc/Tab] Back │ {}", scroll_info)),
                    ResetColor
                ).map_err(|e| e.to_string())?;
                
                self.stdout.flush().map_err(|e| e.to_string())?;
            }
            
            // Handle input
            if let Ok(Event::Key(key)) = read() {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Esc | KeyCode::Tab => {
                        return Ok(());
                    }
                    KeyCode::Up => {
                        scroll_offset = scroll_offset.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        if scroll_offset + visible_lines < lines.len() {
                            scroll_offset += 1;
                        }
                    }
                    KeyCode::PageUp => {
                        scroll_offset = scroll_offset.saturating_sub(visible_lines);
                    }
                    KeyCode::PageDown => {
                        scroll_offset = (scroll_offset + visible_lines)
                            .min(lines.len().saturating_sub(visible_lines));
                    }
                    KeyCode::Home => {
                        scroll_offset = 0;
                    }
                    KeyCode::End => {
                        scroll_offset = lines.len().saturating_sub(visible_lines);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new("MadTyping".to_string(), "untitled".to_string())
    }
}

/// Truncate a line for display, adding ellipsis if too long.
fn truncate_line(line: &str, max_len: usize) -> String {
    let chars: Vec<char> = line.chars().collect();
    if chars.len() > max_len {
        format!("{}...", chars[..max_len].iter().collect::<String>())
    } else {
        line.to_string()
    }
}

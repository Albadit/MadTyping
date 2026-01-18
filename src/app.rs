//! Application state management for MadTyping
//!
//! Contains the App struct which manages file list state,
//! selection, filtering, and error handling.

use crate::files::{self, TextFile};

/// Application state for the CLI.
/// 
/// Manages the list of discovered files, current selection,
/// search filtering, and error messages.
pub struct App {
    /// All discovered text files
    files: Vec<TextFile>,
    /// Indices into `files` that match the current search query
    filtered_indices: Vec<usize>,
    /// Currently selected index in `filtered_indices`
    selected_index: usize,
    /// Current search query
    search_query: String,
    /// Error message to display (if any)
    error_message: Option<String>,
}

impl App {
    /// Create a new App instance with the given files.
    pub fn new(files: Vec<TextFile>) -> Self {
        let filtered_indices: Vec<usize> = (0..files.len()).collect();
        Self {
            files,
            filtered_indices,
            selected_index: 0,
            search_query: String::new(),
            error_message: None,
        }
    }

    /// Update filtered indices based on search query.
    fn update_filter(&mut self) {
        let query = self.search_query.to_lowercase();
        self.filtered_indices = self.files
            .iter()
            .enumerate()
            .filter(|(_, f)| query.is_empty() || f.name.to_lowercase().contains(&query))
            .map(|(i, _)| i)
            .collect();
        
        // Reset selection if out of bounds
        if self.selected_index >= self.filtered_indices.len() {
            self.selected_index = 0;
        }
    }

    /// Move selection up (wraps to bottom).
    pub fn move_up(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.filtered_indices.len().saturating_sub(1);
        }
    }

    /// Move selection down (wraps to top).
    pub fn move_down(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        if self.selected_index < self.filtered_indices.len().saturating_sub(1) {
            self.selected_index += 1;
        } else {
            self.selected_index = 0;
        }
    }

    /// Get the currently selected file.
    pub fn get_selected(&self) -> Option<&TextFile> {
        self.filtered_indices
            .get(self.selected_index)
            .and_then(|&i| self.files.get(i))
    }

    /// Get filtered files for display.
    pub fn filtered_files(&self) -> Vec<&TextFile> {
        self.filtered_indices
            .iter()
            .filter_map(|&i| self.files.get(i))
            .collect()
    }

    /// Get selected index.
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Set an error message to display.
    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
    }

    /// Clear the error message.
    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    /// Refresh the file list by re-discovering files.
    /// Returns the number of files changed (added + removed).
    pub fn refresh_files(&mut self) -> Result<usize, String> {
        let new_files = files::discover()?;
        let new_count = new_files.len();
        let old_count = self.files.len();
        self.files = new_files;
        self.search_query.clear();
        self.filtered_indices = (0..self.files.len()).collect();
        self.selected_index = 0;
        Ok(new_count.saturating_sub(old_count.min(new_count)) + old_count.saturating_sub(new_count.min(old_count)))
    }

    /// Get the current error message.
    pub fn get_error(&self) -> Option<&String> {
        self.error_message.as_ref()
    }

    /// Add a character to search query.
    pub fn add_search_char(&mut self, c: char) {
        self.search_query.push(c);
        self.update_filter();
    }

    /// Remove last character from search query.
    pub fn remove_search_char(&mut self) {
        self.search_query.pop();
        self.update_filter();
    }

    /// Get the search query.
    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    /// Get count of filtered files.
    pub fn filtered_count(&self) -> usize {
        self.filtered_indices.len()
    }

    /// Check if the search query is empty.
    pub fn is_search_empty(&self) -> bool {
        self.search_query.is_empty()
    }

    /// Get total file count.
    pub fn total_count(&self) -> usize {
        self.files.len()
    }
}

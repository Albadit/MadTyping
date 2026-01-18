//! File discovery and management for MadTyping
//!
//! Handles discovering text files (.txt, .md) from the executable's directory
//! and loading their contents.

use std::{
    env,
    fs,
    path::PathBuf,
};

use crate::config::SUPPORTED_EXTENSIONS;

/// Represents a discovered text file with its contents.
#[derive(Clone, Debug)]
pub struct TextFile {
    /// The filename (e.g., "messages.txt")
    pub name: String,
    /// Full path to the file
    pub path: PathBuf,
    /// Non-empty lines from the file (trimmed)
    pub lines: Vec<String>,
}

impl TextFile {
    /// Create a new TextFile from a path, reading and parsing its contents.
    /// Returns None if the file can't be read or has no non-empty lines.
    pub fn from_path(path: PathBuf) -> Option<Self> {
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        match fs::read_to_string(&path) {
            Ok(contents) => {
                let lines: Vec<String> = contents
                    .lines()
                    .map(|l| l.trim())
                    .filter(|l| !l.is_empty())
                    .map(|l| l.to_string())
                    .collect();
                
                if lines.is_empty() {
                    None
                } else {
                    Some(Self { name, path, lines })
                }
            }
            Err(e) => {
                eprintln!("Warning: Could not read {}: {}", name, e);
                None
            }
        }
    }

    /// Get the number of lines in this file.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

/// Discover all supported text files from the executable's directory.
/// 
/// Scans the directory containing the executable for .txt and .md files,
/// reads their contents, and returns a list of TextFile objects.
/// 
/// # Errors
/// Returns an error if:
/// - The executable path cannot be determined
/// - The directory cannot be read
/// - No valid text files are found
pub fn discover() -> Result<Vec<TextFile>, String> {
    let exe_dir = get_exe_directory()?;
    let mut files: Vec<TextFile> = Vec::new();

    let entries = fs::read_dir(&exe_dir)
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        
        if path.is_file() && is_supported_extension(&path) {
            if let Some(text_file) = TextFile::from_path(path) {
                files.push(text_file);
            }
        }
    }

    if files.is_empty() {
        return Err(format!(
            "No .txt or .md files with content found in directory: {}",
            exe_dir.display()
        ));
    }

    // Sort files alphabetically by name for consistent ordering
    files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(files)
}

/// Get the directory containing the executable.
fn get_exe_directory() -> Result<PathBuf, String> {
    let exe_path = env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?;
    
    exe_path.parent()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| "Failed to get executable directory".to_string())
}

/// Check if a file has a supported extension.
fn is_supported_extension(path: &PathBuf) -> bool {
    path.extension()
        .map(|ext| {
            let ext_lower = ext.to_string_lossy().to_lowercase();
            SUPPORTED_EXTENSIONS.contains(&ext_lower.as_str())
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_extensions() {
        assert!(is_supported_extension(&PathBuf::from("test.txt")));
        assert!(is_supported_extension(&PathBuf::from("test.md")));
        assert!(is_supported_extension(&PathBuf::from("test.TXT")));
        assert!(is_supported_extension(&PathBuf::from("test.MD")));
        assert!(!is_supported_extension(&PathBuf::from("test.rs")));
        assert!(!is_supported_extension(&PathBuf::from("test")));
    }
}

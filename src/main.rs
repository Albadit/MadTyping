//! MadTyping - Entry Point
//!
//! A League of Legends chat automation tool that reads text files
//! and types their contents into the game chat.

use mad_typing::{
    App, Cli, 
    discover_files, init_logging, log,
    DEFAULT_HEADER_NAME, DEFAULT_WINDOW_TITLE,
};

/// Run the application.
fn run_app() -> Result<(), String> {
    init_logging();
    log("=== MadTyping Starting ===");
    
    log("Scanning for .txt and .md files...");
    println!("Scanning for .txt and .md files...");
    
    let files = discover_files()?;
    log(&format!("Found {} files", files.len()));
    println!("Found {} files.", files.len());

    let mut cli = Cli::new(
        DEFAULT_HEADER_NAME.to_string(),
        DEFAULT_WINDOW_TITLE.to_string(),
    );
    
    log("Cli created, initializing...");
    let mut app = App::new(files);
    
    cli.init()?;
    log("Cli initialized, running main loop...");
    
    let result = cli.run(&mut app);
    cli.cleanup()?;

    log("MadTyping exited");
    println!("MadTyping exited. Goodbye!");
    result
}

fn main() {
    if let Err(e) = run_app() {
        eprintln!("\n❌ Error: {}", e);
        eprintln!("\nMake sure:");
        eprintln!("  1. There are .txt or .md files in the same directory as the executable");
        eprintln!("  2. The files contain non-empty lines");
        eprintln!("  3. You have proper permissions to read the files");
        std::process::exit(1);
    }
}

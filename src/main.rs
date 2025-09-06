// src/main.rs

use clap::{Parser, Subcommand};

/// CLI definition
#[derive(Parser)]
#[command(name = "mechsound", about = "Mechanical keyboard sound CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Play sound on every keypress
    Listen,
    /// Play sound when a specific keyword is typed
    Keyword { word: String },
    /// Test sound playback
    Test,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Listen => mechsound::listen_mode(),
        Commands::Keyword { word } => mechsound::keyword_mode(&word),
        Commands::Test => mechsound::test_sound(),
    }
}

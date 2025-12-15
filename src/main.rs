use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};
use std::fs::File;
use std::io::{BufRead, BufReader};

// Search for a pattern and display the lines
#[derive(Parser)]
struct Cli {
    // Pattern to look for in file
    pattern: String,
    // Read file path
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");

    // Parse into the Cli struct
    let args = Cli::parse();

    // Open file and propagate errors automatically
    let file = File::open(&args.path)
        .with_context(|| format!("could not open file `{}`", args.path.display()))?;

    // Wrap file in BufReader
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    // Display error when unable to read file
    reader
        .read_line(&mut line)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in reader.lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

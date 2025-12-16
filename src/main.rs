use crate::find_matches::search_file;
use clap::Parser;
mod find_matches;
use anyhow::Result;

// Search for a pattern and display the lines
#[derive(Parser)]
struct Cli {
    // Pattern to look for in file
    pattern: String,
    // Read file path
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // Parse into the Cli struct
    let args = Cli::parse();
    let matches = search_file(&args.path, &args.pattern);

    if let Ok(line) = matches {
        println!("{:?}", line);
    }

    Ok(())
}

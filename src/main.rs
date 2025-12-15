use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Search for a pattern and display the lines
#[derive(Parser)]
struct Cli {
    // Pattern to look for in file
    pattern: String,
    // Read file path
    path: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    // Parse into the Cli struct
    let args = Cli::parse();
    // Open file
    let file = File::open(&args.path)?;
    // Wrap file in BufReader
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

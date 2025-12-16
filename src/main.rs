use anyhow::{Context, Result};
use clap::Parser;
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

fn contains_pattern(line: &str, pattern: &str) -> bool {
    line.contains(pattern)
}

fn search_file(path: &std::path::Path, pattern: &str) -> Result<Vec<String>> {
    // Open file and propagate errors automatically
    let file =
        File::open(path).with_context(|| format!("could not open file `{}`", path.display()))?;

    // Wrap file in BufReader
    let reader = BufReader::new(file);

    let mut results = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if contains_pattern(&line, pattern) {
            results.push(line);
        }
    }
    Ok(results)
}

#[test]
fn test_pattern() {
    assert!(contains_pattern("Hello World", "Hello"));
    assert!(!contains_pattern("Hello World", "bye"));
}

fn main() -> Result<()> {
    // Parse into the Cli struct
    let args = Cli::parse();
    let matches = search_file(&args.path, &args.pattern);

    for line in matches {
        println!("{:?}", line);
    }

    Ok(())
}

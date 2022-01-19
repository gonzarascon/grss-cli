#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let f = File::open(&args.path)
        .with_context(|| format!("could not open {}", args.path.display()))?;
    let mut reader = BufReader::new(f);
    let lines = reader.lines();

    let matching_lines = lines
        .map(|l| l.unwrap())
        .filter(|l| l.contains(&args.pattern))
        .next();

    match matching_lines {
        Some(line) => {
            println!("{}", line);
        }
        None => {
            println!("No lines found");
        }
    }

    Ok(())
}

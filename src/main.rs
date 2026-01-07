use std::path::PathBuf;

use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(
    name = "recursive_grep",
    about = "Recursively search files in a directory for a substring or regex.",
    version
)]
struct Args {
    /// Search string
    pattern: String,
    /// Search directory
    path: PathBuf,
    /// Stop after printing # lines (default: infinite)
    #[arg(long = "max-lines")]
    max_lines: Option<usize>,
    /// Case-insensitive search
    #[arg(long)]
    ignore_case: bool,
    /// Print only # of matches
    #[arg(long)]
    count: bool,
    /// Search string as regex
    #[arg(long)]
    regex: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    for entry in WalkDir::new(&args.path) {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Failed to read entry: {err}");
                continue;
            }
        };

        if entry.file_type().is_file() {
            println!("{}", entry.path().display());
        }
    }

    Ok(())
}

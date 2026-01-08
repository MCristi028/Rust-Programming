use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;
use regex::RegexBuilder;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(
    name = "recursive_grep",
    about = "Recursively search files in a directory for a substring or regex.",
    version
)]
struct Args {
    /// Search string / regex pattern
    pattern: String,
    /// Search directory
    path: PathBuf,
    /// Stop after printing # lines (default: inf)
    #[arg(long = "max-lines")]
    max_lines: Option<usize>,
    /// Case-insensitive search
    #[arg(long)]
    ignore_case: bool,
    /// Print only # of matches
    #[arg(long)]
    count: bool,
    /// Treat string as regex
    #[arg(long)]
    regex: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let compiled = if args.regex {
        Some(
            RegexBuilder::new(&args.pattern)
                .case_insensitive(args.ignore_case)
                .build()?,
        )
    } else {
        None
    };

    let pattern_lower = if !args.regex && args.ignore_case {
        Some(args.pattern.to_lowercase())
    } else {
        None
    };

    let mut printed_total = 0usize; // current # of printed lines

    for entry in WalkDir::new(&args.path) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let file = match File::open(entry.path()) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let mut reader = BufReader::new(file);

        let mut line_buf = String::new();
        let mut line_no = 0usize;

        let mut matches_in_file = 0usize;
        let mut not_utf8 = false;

        loop {
            line_buf.clear();

            let bytes_read = match reader.read_line(&mut line_buf) {
                Ok(n) => n,
                Err(_) => {
                    not_utf8 = true;
                    break;
                }
            };

            if bytes_read == 0 {
                break;
            }

            line_no += 1;

            let line = line_buf.trim_end();

            let is_match = if let Some(re) = &compiled {
                re.is_match(line)
            } else if let Some(pattern) = &pattern_lower {
                line.to_lowercase().contains(pattern)
            } else {
                line.contains(&args.pattern)
            };

            if !is_match {
                continue;
            }

            matches_in_file += 1;

            if args.count {
                continue;
            }

            println!("{}:{}:{}", entry.path().display(), line_no, line);
            printed_total += 1;

            if let Some(limit) = args.max_lines
                && printed_total >= limit
            {
                return Ok(());
            }
        }

        // If we hit a UTF-8 decoding error, skip the whole file (print nothing).
        if not_utf8 {
            continue;
        }

        if args.count && matches_in_file > 0 {
            println!("{}:{}", entry.path().display(), matches_in_file);
        }
    }

    Ok(())
}

//! `file-inspector <path> [--duplicates]`
//!
//! Exit codes: 0 everything inspected, 1 some paths could not be read, 2 usage error.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use file_inspector::{duplicates, inspect, wasted_bytes};

/// Report the size and SHA-256 of every file under a directory.
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Directory (or single file) to inspect.
    path: PathBuf,

    /// Show only groups of files with identical contents.
    #[arg(long)]
    duplicates: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let report = inspect(&args.path);

    if args.duplicates {
        for group in duplicates(&report.files) {
            println!(
                "{} ({} bytes each, {} bytes wasted)",
                group[0].sha256,
                group[0].size,
                wasted_bytes(&group)
            );
            for f in group {
                println!("  {}", f.path.display());
            }
        }
    } else {
        for f in &report.files {
            println!("{:>12}  {}  {}", f.size, f.sha256, f.path.display());
        }
    }

    for path in &report.skipped {
        eprintln!("skipped (not a regular file): {}", path.display());
    }
    for err in &report.errors {
        eprintln!("error: {err}");
    }

    let total: u64 = report.files.iter().map(|f| f.size).sum();
    eprintln!(
        "{} files, {} bytes, {} errors",
        report.files.len(),
        total,
        report.errors.len()
    );

    if report.errors.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

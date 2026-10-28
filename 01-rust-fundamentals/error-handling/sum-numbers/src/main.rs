//! `sum-numbers <file>`: prints the sum of the integers in `file`.
//!
//! Exit codes: 0 success, 1 the input could not be summed, 2 usage error.

use std::env;
use std::error::Error;
use std::path::Path;
use std::process::ExitCode;

use sum_numbers::sum_file;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let [_, path] = args.as_slice() else {
        eprintln!("usage: sum-numbers <file>");
        return ExitCode::from(2);
    };

    match sum_file(Path::new(path)) {
        Ok(total) => {
            println!("{total}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("error: {err}");
            // Walk the chain of causes: SumError -> io::Error / ParseIntError.
            let mut cause = err.source();
            while let Some(c) = cause {
                eprintln!("  caused by: {c}");
                cause = c.source();
            }
            ExitCode::FAILURE
        }
    }
}

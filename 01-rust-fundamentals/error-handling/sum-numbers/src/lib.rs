//! Sum the integers in a text file, one per line, without ever panicking on bad input.
//!
//! The exercise in three steps (see ../README.md):
//! 1. `unwrap()` everywhere: crashes on the first problem.
//! 2. `Box<dyn Error>` and `?`: no crash, but callers can't tell errors apart.
//! 3. A custom error enum (this file): every failure is a named, testable case.

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;
use std::path::Path;

/// Everything that can go wrong while summing a file.
#[derive(Debug)]
pub enum SumError {
    /// The file could not be read (missing, no permission, not UTF-8, ...).
    Io(io::Error),
    /// A line was not a valid integer. `line` is 1-based.
    Parse {
        line: usize,
        text: String,
        source: ParseIntError,
    },
    /// The file contained no numbers at all.
    Empty,
    /// The running total no longer fits in an `i64`.
    Overflow { line: usize },
}

impl fmt::Display for SumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SumError::Io(_) => write!(f, "could not read input"),
            SumError::Parse { line, text, .. } => {
                write!(f, "line {line}: {text:?} is not an integer")
            }
            SumError::Empty => write!(f, "input contains no numbers"),
            SumError::Overflow { line } => write!(f, "line {line}: total overflows i64"),
        }
    }
}

impl Error for SumError {
    /// Exposes the underlying cause, so callers can print the whole chain.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SumError::Io(e) => Some(e),
            SumError::Parse { source, .. } => Some(source),
            SumError::Empty | SumError::Overflow { .. } => None,
        }
    }
}

/// Lets `?` turn an `io::Error` into a `SumError` automatically.
impl From<io::Error> for SumError {
    fn from(e: io::Error) -> Self {
        SumError::Io(e)
    }
}

// There is deliberately no `From<ParseIntError>`: a bare ParseIntError does not
// know which line it came from, so `parse_numbers` adds that context with
// `map_err` instead. See "Why not From<ParseIntError>?" in ../README.md.

/// Parses one integer per line. Blank lines and lines starting with `#` are skipped.
pub fn parse_numbers(text: &str) -> Result<Vec<i64>, SumError> {
    let mut numbers = Vec::new();
    for (index, raw) in text.lines().enumerate() {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let n = trimmed.parse::<i64>().map_err(|source| SumError::Parse {
            line: index + 1,
            text: trimmed.to_string(),
            source,
        })?;
        numbers.push(n);
    }
    Ok(numbers)
}

/// Adds the numbers, reporting overflow instead of wrapping or panicking.
pub fn checked_total(text: &str) -> Result<i64, SumError> {
    let numbers = parse_numbers(text)?;
    if numbers.is_empty() {
        return Err(SumError::Empty);
    }
    let mut total: i64 = 0;
    for (i, n) in numbers.iter().enumerate() {
        total = total
            .checked_add(*n)
            .ok_or(SumError::Overflow { line: i + 1 })?;
    }
    Ok(total)
}

/// Reads `path` and sums it. `?` on `read_to_string` uses `From<io::Error>`.
pub fn sum_file(path: &Path) -> Result<i64, SumError> {
    let text = fs::read_to_string(path)?;
    checked_total(&text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_valid_input_skipping_blanks_and_comments() {
        let text = "10\n\n# a comment\n-3\n  5  \n";
        assert_eq!(checked_total(text).unwrap(), 12);
    }

    #[test]
    fn bad_line_reports_line_number_and_text() {
        let err = checked_total("1\n2\nthree\n").unwrap_err();
        match err {
            SumError::Parse { line, text, .. } => {
                assert_eq!(line, 3);
                assert_eq!(text, "three");
            }
            other => panic!("expected Parse, got {other:?}"),
        }
    }

    #[test]
    fn parse_error_keeps_its_source() {
        let err = checked_total("12x\n").unwrap_err();
        let source = err.source().expect("Parse should have a source");
        assert_eq!(source.to_string(), "invalid digit found in string");
    }

    #[test]
    fn empty_input_is_an_error_not_zero() {
        assert!(matches!(checked_total(""), Err(SumError::Empty)));
        assert!(matches!(checked_total("# only\n\n"), Err(SumError::Empty)));
    }

    #[test]
    fn overflow_is_detected() {
        let text = format!("{}\n1\n", i64::MAX);
        assert!(matches!(
            checked_total(&text),
            Err(SumError::Overflow { line: 2 })
        ));
    }

    #[test]
    fn value_out_of_range_is_a_parse_error() {
        let err = checked_total("99999999999999999999\n").unwrap_err();
        assert!(matches!(err, SumError::Parse { line: 1, .. }));
        assert!(err.source().unwrap().to_string().contains("too large"));
    }

    #[test]
    fn display_messages_are_human_readable() {
        let err = checked_total("abc\n").unwrap_err();
        assert_eq!(err.to_string(), r#"line 1: "abc" is not an integer"#);
        assert_eq!(SumError::Empty.to_string(), "input contains no numbers");
    }
}

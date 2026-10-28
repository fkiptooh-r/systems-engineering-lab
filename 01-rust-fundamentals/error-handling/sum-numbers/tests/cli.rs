//! Integration tests: run the real binary against real files, check output and exit codes.

use std::fs;
use std::process::Command;

use tempfile::TempDir;

/// Path to the compiled binary, provided by Cargo for integration tests.
const BIN: &str = env!("CARGO_BIN_EXE_sum-numbers");

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(BIN).args(args).output().expect("binary runs");
    (
        out.status.code().expect("exited normally"),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn prints_the_sum_and_exits_zero() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("ok.txt");
    fs::write(&path, "1\n2\n3\n").unwrap();

    let (code, stdout, stderr) = run(&[path.to_str().unwrap()]);
    assert_eq!(code, 0);
    assert_eq!(stdout.trim(), "6");
    assert!(stderr.is_empty());
}

#[test]
fn missing_file_reports_the_os_error_chain() {
    let (code, stdout, stderr) = run(&["/definitely/not/here.txt"]);
    assert_eq!(code, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains("could not read input"), "{stderr}");
    assert!(stderr.contains("No such file or directory"), "{stderr}");
}

#[test]
fn bad_line_reports_line_and_cause() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("bad.txt");
    fs::write(&path, "1\nnope\n").unwrap();

    let (code, _, stderr) = run(&[path.to_str().unwrap()]);
    assert_eq!(code, 1);
    assert!(
        stderr.contains(r#"line 2: "nope" is not an integer"#),
        "{stderr}"
    );
    assert!(stderr.contains("invalid digit found in string"), "{stderr}");
}

#[test]
fn wrong_argument_count_is_a_usage_error() {
    let (code, _, stderr) = run(&[]);
    assert_eq!(code, 2);
    assert!(stderr.contains("usage:"), "{stderr}");
}

//! Integration tests: build a real directory tree, run the real binary.

use std::fs::{self, File, Permissions};
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::Path;
use std::process::Command;

use tempfile::TempDir;

const BIN: &str = env!("CARGO_BIN_EXE_file-inspector");
const ABC: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

fn run(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(BIN).args(args).output().expect("binary runs");
    (
        out.status.code().expect("exited normally"),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// root/
///   a.txt          "abc"
///   nested/b.txt   "abc"      (duplicate of a.txt)
///   nested/deeper/c.txt "hello"
///   empty.txt      ""
fn fixture() -> TempDir {
    let dir = TempDir::new().unwrap();
    let root = dir.path();
    fs::create_dir_all(root.join("nested/deeper")).unwrap();
    fs::write(root.join("a.txt"), "abc").unwrap();
    fs::write(root.join("nested/b.txt"), "abc").unwrap();
    fs::write(root.join("nested/deeper/c.txt"), "hello").unwrap();
    File::create(root.join("empty.txt")).unwrap();
    dir
}

/// Root ignores permission bits, so permission tests are meaningless when run as root.
fn permissions_enforced(probe: &Path) -> bool {
    File::open(probe).is_err()
}

#[test]
fn lists_every_file_recursively_with_size_and_hash() {
    let dir = fixture();
    let (code, stdout, stderr) = run(&[dir.path().to_str().unwrap()]);

    assert_eq!(code, 0, "{stderr}");
    assert_eq!(stdout.lines().count(), 4, "{stdout}");
    let a = stdout.lines().find(|l| l.ends_with("/a.txt")).unwrap();
    assert!(a.contains(ABC) && a.trim_start().starts_with("3 "), "{a}");
    assert!(stderr.contains("4 files, 11 bytes, 0 errors"), "{stderr}");
}

#[test]
fn duplicates_flag_groups_identical_files() {
    let dir = fixture();
    let (code, stdout, _) = run(&[dir.path().to_str().unwrap(), "--duplicates"]);

    assert_eq!(code, 0);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 3, "{stdout}");
    assert!(lines[0].starts_with(ABC));
    assert!(lines[0].contains("3 bytes wasted"));
    assert!(lines[1].ends_with("/a.txt") && lines[2].ends_with("/nested/b.txt"));
}

#[test]
fn unreadable_file_is_reported_and_the_walk_continues() {
    let dir = fixture();
    let secret = dir.path().join("secret.txt");
    fs::write(&secret, "top secret").unwrap();
    fs::set_permissions(&secret, Permissions::from_mode(0o000)).unwrap();
    if !permissions_enforced(&secret) {
        eprintln!("running as root; skipping permission test");
        return;
    }

    let (code, stdout, stderr) = run(&[dir.path().to_str().unwrap()]);

    assert_eq!(code, 1, "partial failure must be visible to scripts");
    assert_eq!(stdout.lines().count(), 4, "other files still listed");
    assert!(
        stderr.contains("cannot read file") && stderr.contains("Permission denied"),
        "{stderr}"
    );
}

#[test]
fn unreadable_directory_is_reported_and_the_walk_continues() {
    let dir = fixture();
    let locked = dir.path().join("locked");
    fs::create_dir(&locked).unwrap();
    fs::write(locked.join("hidden.txt"), "x").unwrap();
    fs::set_permissions(&locked, Permissions::from_mode(0o000)).unwrap();
    if fs::read_dir(&locked).is_ok() {
        return; // running as root
    }

    let (code, stdout, stderr) = run(&[dir.path().to_str().unwrap()]);
    // Restore so TempDir can clean up.
    fs::set_permissions(&locked, Permissions::from_mode(0o755)).unwrap();

    assert_eq!(code, 1);
    assert_eq!(stdout.lines().count(), 4);
    assert!(stderr.contains("cannot list directory"), "{stderr}");
}

#[test]
fn symlinks_are_not_followed() {
    let dir = fixture();
    // A link back to the root would loop forever if followed.
    symlink(dir.path(), dir.path().join("loop")).unwrap();

    let (code, stdout, stderr) = run(&[dir.path().to_str().unwrap()]);
    assert_eq!(code, 0);
    assert_eq!(stdout.lines().count(), 4);
    assert!(stderr.contains("skipped (not a regular file)"), "{stderr}");
}

#[test]
fn missing_path_is_an_error() {
    let (code, stdout, stderr) = run(&["/definitely/not/here"]);
    assert_eq!(code, 1);
    assert!(stdout.is_empty());
    assert!(stderr.contains("No such file or directory"), "{stderr}");
}

#[test]
fn no_arguments_is_a_usage_error() {
    let (code, _, stderr) = run(&[]);
    assert_eq!(code, 2);
    assert!(stderr.contains("Usage:"), "{stderr}");
}

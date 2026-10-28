//! Walk a directory tree, report each file's size and SHA-256, and find duplicates.
//!
//! Failures on individual files or directories are collected, not fatal: one
//! unreadable file must not hide the results for the thousands of others.

use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

/// Read buffer size. Files are hashed in chunks of this size, so memory use stays
/// constant no matter how large the file is.
pub const CHUNK_SIZE: usize = 64 * 1024;

/// A regular file that was read and hashed successfully.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    /// Lowercase hex SHA-256 of the contents.
    pub sha256: String,
}

/// What the inspector was doing when an I/O error happened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    ListDirectory,
    ReadMetadata,
    ReadFile,
}

/// A failure on one path. The walk continues past it.
#[derive(Debug)]
pub struct InspectError {
    pub path: PathBuf,
    pub operation: Operation,
    pub source: io::Error,
}

impl fmt::Display for InspectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let action = match self.operation {
            Operation::ListDirectory => "cannot list directory",
            Operation::ReadMetadata => "cannot read metadata of",
            Operation::ReadFile => "cannot read file",
        };
        write!(f, "{action} {}: {}", self.path.display(), self.source)
    }
}

impl std::error::Error for InspectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// The result of inspecting a tree: everything that worked, and everything that didn't.
#[derive(Debug, Default)]
pub struct Report {
    /// Sorted by path, so output is deterministic.
    pub files: Vec<FileEntry>,
    pub errors: Vec<InspectError>,
    /// Symlinks and special files (sockets, FIFOs, devices) that were not followed.
    pub skipped: Vec<PathBuf>,
}

/// Hashes any reader in fixed-size chunks.
pub fn hash_reader<R: Read>(mut reader: R) -> io::Result<String> {
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; CHUNK_SIZE];
    loop {
        let n = match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            // A signal interrupted the read syscall; nothing was lost, try again.
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        hasher.update(&buf[..n]);
    }
    Ok(to_hex(&hasher.finalize()))
}

/// Opens and hashes one file.
pub fn hash_file(path: &Path) -> io::Result<String> {
    hash_reader(File::open(path)?)
}

fn to_hex(bytes: &[u8]) -> String {
    use fmt::Write;
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        })
}

/// Walks `root` recursively and hashes every regular file.
///
/// Uses an explicit stack instead of recursion, so a very deep tree cannot overflow
/// the call stack. Symlinks are never followed, which also rules out symlink loops.
pub fn inspect(root: &Path) -> Report {
    let mut report = Report::default();
    let mut pending = vec![root.to_path_buf()];

    while let Some(path) = pending.pop() {
        // symlink_metadata (lstat) describes the link itself, not its target.
        let meta = match fs::symlink_metadata(&path) {
            Ok(m) => m,
            Err(source) => {
                report.errors.push(InspectError {
                    path,
                    operation: Operation::ReadMetadata,
                    source,
                });
                continue;
            }
        };

        let kind = meta.file_type();
        if kind.is_dir() {
            match fs::read_dir(&path) {
                Ok(entries) => {
                    for entry in entries {
                        match entry {
                            Ok(e) => pending.push(e.path()),
                            Err(source) => report.errors.push(InspectError {
                                path: path.clone(),
                                operation: Operation::ListDirectory,
                                source,
                            }),
                        }
                    }
                }
                Err(source) => report.errors.push(InspectError {
                    path,
                    operation: Operation::ListDirectory,
                    source,
                }),
            }
        } else if kind.is_file() {
            match hash_file(&path) {
                Ok(sha256) => report.files.push(FileEntry {
                    path,
                    size: meta.len(),
                    sha256,
                }),
                Err(source) => report.errors.push(InspectError {
                    path,
                    operation: Operation::ReadFile,
                    source,
                }),
            }
        } else {
            report.skipped.push(path);
        }
    }

    report.files.sort_by(|a, b| a.path.cmp(&b.path));
    report.errors.sort_by(|a, b| a.path.cmp(&b.path));
    report.skipped.sort();
    report
}

/// Groups files with identical contents. Only groups of two or more are returned,
/// largest wasted space first.
pub fn duplicates(files: &[FileEntry]) -> Vec<Vec<&FileEntry>> {
    let mut by_hash: HashMap<&str, Vec<&FileEntry>> = HashMap::new();
    for f in files {
        by_hash.entry(f.sha256.as_str()).or_default().push(f);
    }
    let mut groups: Vec<Vec<&FileEntry>> = by_hash.into_values().filter(|g| g.len() > 1).collect();
    groups.sort_by(|a, b| {
        wasted_bytes(b)
            .cmp(&wasted_bytes(a))
            .then_with(|| a[0].path.cmp(&b[0].path))
    });
    groups
}

/// Bytes that could be freed by keeping one copy of the group.
pub fn wasted_bytes(group: &[&FileEntry]) -> u64 {
    group
        .first()
        .map_or(0, |f| f.size * (group.len() as u64 - 1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // Known SHA-256 values (FIPS 180-2 test vectors).
    const EMPTY: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    const ABC: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

    #[test]
    fn hashes_known_vectors() {
        assert_eq!(hash_reader(Cursor::new(b"")).unwrap(), EMPTY);
        assert_eq!(hash_reader(Cursor::new(b"abc")).unwrap(), ABC);
    }

    #[test]
    fn chunked_hash_matches_one_shot_hash() {
        // 2.5 chunks, so the loop runs several times and ends on a partial read.
        let data: Vec<u8> = (0..CHUNK_SIZE * 5 / 2).map(|i| (i % 251) as u8).collect();
        let expected = to_hex(&Sha256::digest(&data));
        assert_eq!(hash_reader(Cursor::new(&data)).unwrap(), expected);
    }

    #[test]
    fn interrupted_reads_are_retried() {
        struct Flaky {
            inner: Cursor<&'static [u8]>,
            interrupted: bool,
        }
        impl Read for Flaky {
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                if !self.interrupted {
                    self.interrupted = true;
                    return Err(io::ErrorKind::Interrupted.into());
                }
                self.inner.read(buf)
            }
        }
        let flaky = Flaky {
            inner: Cursor::new(b"abc"),
            interrupted: false,
        };
        assert_eq!(hash_reader(flaky).unwrap(), ABC);
    }

    fn entry(path: &str, size: u64, sha256: &str) -> FileEntry {
        FileEntry {
            path: path.into(),
            size,
            sha256: sha256.into(),
        }
    }

    #[test]
    fn duplicates_groups_by_hash_largest_waste_first() {
        let files = vec![
            entry("a", 10, "h1"),
            entry("b", 10, "h1"),
            entry("c", 100, "h2"),
            entry("d", 100, "h2"),
            entry("e", 5, "h3"),
        ];
        let groups = duplicates(&files);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0][0].sha256, "h2");
        assert_eq!(wasted_bytes(&groups[0]), 100);
        assert_eq!(wasted_bytes(&groups[1]), 10);
    }

    #[test]
    fn no_duplicates_means_no_groups() {
        let files = vec![entry("a", 1, "x"), entry("b", 1, "y")];
        assert!(duplicates(&files).is_empty());
    }
}

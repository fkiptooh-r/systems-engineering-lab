# file-inspector

Issue: #3 · Phase 1 project

```text
$ file-inspector /tmp/fi
  1073741824  49bc20df15e412a64472421e13fe86ff1c5165e18b2afccf160d4dc19fe68a14  /tmp/fi/big.bin
     1048576  a313b0c258e0135403f24f2b151659d8e290f8377a373c60d97017d8103d88f7  /tmp/fi/copy.bin
     1048576  a313b0c258e0135403f24f2b151659d8e290f8377a373c60d97017d8103d88f7  /tmp/fi/small.bin
3 files, 1075838976 bytes, 0 errors

$ file-inspector /tmp/fi --duplicates
a313b0c2...d88f7 (1048576 bytes each, 1048576 bytes wasted)
  /tmp/fi/copy.bin
  /tmp/fi/small.bin
```

Run from `01-rust-fundamentals/`: `cargo run -p file-inspector -- <path> [--duplicates]`.
Tests: `cargo test -p file-inspector`.

## 1. Problem

Given a directory, report every file's size and SHA-256 hash and find files with identical
contents. Real directories contain things that break naive tools: unreadable files, locked
directories, symlink loops, and files too large to fit in memory. The tool must keep going
past every one of them and still tell the caller something went wrong.

This is the integrity-checking building block for the offline content server
(`10-capstone-offline-platform/`): verifying that lessons copied from a USB drive arrived
intact is the same operation.

## 2. Concepts

| Concept | Where |
|---|---|
| Ownership of paths moving through a work stack | `inspect`: `pending.pop()` moves each `PathBuf` out; it is moved again into `FileEntry` or `InspectError` |
| Borrowing to avoid copies | `duplicates` groups `&FileEntry` by `&str` hash; no file entry is cloned |
| Error handling without stopping | `Report { files, errors, skipped }` collects failures instead of returning on the first one |
| Custom error with context | `InspectError { path, operation, source }` says *what* failed, *where*, and *why* |
| Constant-memory I/O | `hash_reader` reads 64 KiB chunks into one reused buffer |
| `lstat` vs `stat` | `fs::symlink_metadata` inspects the link itself, so links are never followed |
| `EINTR` | `hash_reader` retries reads that a signal interrupted |
| Exit codes | 0 all good, 1 partial failure, 2 usage error (from `clap`) |

## 3. Implementation

- **Walk:** an explicit stack (`Vec<PathBuf>`) instead of recursion. Recursion depth would
  equal directory depth; a stack on the heap cannot overflow the call stack. Written with
  `std::fs` only, as the issue suggested, to see what crates like `walkdir` handle for you.
- **Symlinks:** skipped and listed on stderr. Following them would need loop detection
  (tracking device + inode pairs); not following them is simpler and safe. Special files
  (FIFOs, sockets, devices) are skipped too: reading a FIFO could block forever.
- **Hashing:** `hash_reader<R: Read>` is generic over any reader, so unit tests feed it
  in-memory `Cursor`s, and one test feeds it a reader that fails with `Interrupted` first.
- **Output:** files sorted by path for deterministic output (directory order from the OS
  is arbitrary). Results on stdout, diagnostics on stderr, so
  `file-inspector dir > hashes.txt` captures clean data.
- **Duplicates:** a `HashMap<&str, Vec<&FileEntry>>` keyed by hash; groups sorted by wasted
  bytes so the biggest savings come first.
- **Crates:** `clap` (argument parsing, `--help`, usage errors), `sha2` (hashing), `tempfile`
  (test fixtures only).

## 4. Verification

**Tests (12):**

- Unit: SHA-256 known vectors (empty string, `"abc"`); chunked hash equals one-shot hash
  for 2.5 chunks of data; `Interrupted` reads are retried; duplicate grouping and ordering.
- Integration (runs the real binary on a real temp tree): recursive listing with sizes and
  hashes; `--duplicates`; a `chmod 000` file is reported as `Permission denied` while the
  other files are still listed and the exit code is 1; same for a `chmod 000` directory;
  a symlink pointing back at the root does not loop; missing path; no arguments → exit 2.
- Permission tests detect when they run as root (root ignores permission bits) and skip
  rather than fail.

**Measured** (release build, WSL2 Ubuntu 24.04, 1 GiB file + two 1 MiB files):

| Check | Result |
|---|---|
| Hash matches `sha256sum` | yes: `49bc20df…8a14` for the 1 GiB file |
| Peak memory (`/usr/bin/time -f %M`) | **2.8 MB** while hashing 1 GiB |
| Time, 1 GiB | 12.7 s vs 6.6 s for `sha256sum` |

## 5. Reflection

- **Memory is constant, speed is not competitive yet.** `sha256sum` is about 2× faster.
  This CPU has no SHA-NI instructions, so the `sha2` crate uses its portable software
  implementation, while coreutils uses hand-optimised assembly. Worth revisiting in
  Phase 6 (profiling): try `RUSTFLAGS="-C target-cpu=native"`, a larger buffer, and a
  profiler to see where time goes, rather than guessing.
- **Collecting errors changes the API shape.** "Return `Result`" was the lesson of #2; here
  the right answer was a `Report` holding successes *and* failures, because one bad file
  must not hide the results for all the others.
- **The OS decides more than I expected:** directory order, whether root bypasses
  permissions, whether a read can be interrupted. Phase 2 (`strace`) will show these
  system calls directly.
- **Next time:** hash only files whose *size* matches another file's when running
  `--duplicates`. A file with a unique size cannot have a duplicate, which would skip most
  of the hashing on a typical directory.

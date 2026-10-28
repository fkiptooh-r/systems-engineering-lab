# Roadmap

A 12-week, zero-hardware path into systems programming with Rust and Linux.
Budget: 5–7 hours per week. Each phase ends with one small, tested, documented project.

Progress is tracked on GitHub: one **milestone** per phase, one **issue** per project
or concept exercise, and one **branch + pull request** per unit of work.

## Workflow

1. Pick the next issue in the current milestone.
2. Branch from `main`: `phase-1/file-inspector`, `docs/...`, `fix/...`.
3. Commit small, working steps. Keep `cargo fmt`, `cargo clippy`, and `cargo test` clean.
4. Open a pull request that closes the issue (`Closes #N`). Review your own diff before merging.
5. Write the project README (Problem → Concepts → Implementation → Verification → Reflection).
6. Add a dated entry to [JOURNAL.md](JOURNAL.md).

## Phase 1 — Rust fundamentals (weeks 1–2)

Folder: [`01-rust-fundamentals/`](01-rust-fundamentals/)

- Ownership, borrowing, and lifetimes; `String` vs `&str`; `Vec`, `HashMap`
- Structs, enums, pattern matching, traits
- Error handling with `Result`, `?`, and custom error types
- Unit and integration tests

**Project:** `cli-tools/file-inspector`: walk a directory, report file sizes and
SHA-256 hashes, find duplicates, and report unreadable or malformed input without panicking.

## Phase 2 — Linux and OS interfaces (weeks 3–4)

Folders: [`00-tooling/`](00-tooling/), [`02-operating-systems/`](02-operating-systems/)

- Processes, environment variables, exit codes, signals
- File descriptors, permissions, `/proc`
- System calls observed with `strace`; debugging with `gdb`

**Project:** a process monitor that reads `/proc` and reports CPU and memory usage per process.

## Phase 3 — Memory and data (weeks 5–6)

Folder: [`03-data-and-memory/`](03-data-and-memory/)

- Stack vs heap, layout, `Box`/`Rc`/`Arc`
- Byte buffers, endianness, binary formats, serialization

**Project:** a binary file parser and log reader for a self-defined record format.

## Phase 4 — Concurrency (weeks 7–8)

Folder: [`04-concurrency/`](04-concurrency/)

- Threads, channels, `Mutex`, `RwLock`, atomics
- Race conditions, deadlocks, backpressure

**Project:** a bounded worker pool with job failure handling and graceful shutdown on SIGINT.

## Phase 5 — Networking (weeks 9–10)

Folder: [`05-networking/`](05-networking/)

- TCP sockets, protocol framing, connection lifecycle
- Concurrent clients; timeouts; packet inspection with Wireshark/tcpdump

**Project:** a TCP server with a length-prefixed protocol serving many concurrent clients.

## Phase 6 — Systems project (weeks 11–12)

Folders: [`06-storage-and-databases/`](06-storage-and-databases/), [`08-performance-and-reliability/`](08-performance-and-reliability/)

- Append-only logs, fsync, crash recovery
- Retries, structured logging, measuring throughput and memory

**Project:** a persistent local job queue with worker threads, retries, and crash recovery.

## Beyond week 12

- [`07-distributed-systems/`](07-distributed-systems/): idempotency, sync after disconnection
- [`09-embedded-and-edge/`](09-embedded-and-edge/): cross-compiling for ARM, QEMU, Raspberry Pi
- [`10-capstone-offline-platform/`](10-capstone-offline-platform/): an offline content server and
  sync engine for low-connectivity schools and farms

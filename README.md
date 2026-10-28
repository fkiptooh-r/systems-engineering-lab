# Systems Engineering Lab

A practical, project-based journey into systems programming, operating systems, networking, concurrency, storage, and distributed systems.

## Goals

- Develop strong fundamentals in Rust, Linux, memory management, and concurrency.
- Understand how operating systems, networks, and storage systems behave under load and failure.
- Build reliable, resource-efficient software through measurement and testing.
- Apply systems engineering principles to offline-first education and edge computing.
- Explore embedded systems and Raspberry Pi deployment without requiring physical hardware at the beginning.

## Learning Principles

- Build small projects instead of collecting tutorials.
- Write tests and document engineering decisions.
- Measure performance rather than relying on assumptions.
- Investigate failure modes and resource constraints.
- Prefer working, understandable implementations over premature complexity.

## Planned Projects

- [ ] File inspection CLI
- [ ] Concurrent worker pool
- [ ] TCP server
- [ ] Append-only log and simple key-value store
- [ ] Offline content server
- [ ] Synchronization engine for disconnected environments

## Project Write-ups

Each substantial project documents:

1. **Problem** — what it solves.
2. **Concepts** — which systems principles it exercises.
3. **Implementation** — what was built, and why.
4. **Verification** — how correctness, performance, and failure cases were tested.
5. **Reflection** — what broke, what was learned, what would change.

## Environment

- Linux via WSL2 (Ubuntu)
- Docker
- Rust and Cargo
- Git
- Debugging, profiling, and testing tools (gdb, strace, perf, Valgrind)

## Progress

This repository is a living engineering notebook. Projects will be added as prerequisites are understood and implementations are tested.

## License

Released under the [MIT License](LICENSE).

# Journal

Short, dated entries. Newest first. Write one after every session, even a bad one.

```markdown
## YYYY-MM-DD — <topic>

**Worked on:** issue/PR link
**Learned:** the one or two concepts that clicked
**Stuck on:** what confused me, and the error message if any
**Next:** the first thing to do next session
```

---

## 2026-10-28 — file-inspector CLI (#3)

**Worked on:** `cli-tools/file-inspector`: recursive walk with an explicit stack, chunked
SHA-256, `--duplicates`, and a `Report` that collects errors instead of stopping.
**Learned:** measured instead of assumed: 2.8 MB peak memory while hashing a 1 GiB file.
`symlink_metadata` (lstat) vs `metadata` (stat) is the difference between safe and an
infinite loop. Root ignores `chmod 000`, so permission tests must detect it.
**Stuck on:** hashing is 2× slower than `sha256sum` (no SHA-NI on this CPU; coreutils uses
optimised assembly). Parked for Phase 6 profiling.
**Next:** #5 tooling notes, starting with `strace` on the `chmod 000` case.

## 2026-10-28 — Error handling (#2)

**Worked on:** `error-handling/sum-numbers`: the same program written with `unwrap`,
`Box<dyn Error>`, then a custom `SumError` enum; unit tests plus integration tests that
run the real binary and check exit codes.
**Learned:** `?` is `return Err(From::from(e))`. `From` suits conversions that need no
context; `map_err` attaches context such as the line number. `checked_add` turns silent
overflow into an error.
**Stuck on:** whether empty input should be `0` or an error. Chose an error: "no data" and
"sums to zero" are different facts.
**Next:** #3, the file-inspector CLI.

## 2026-10-28 — Ownership and borrowing (#1)

**Worked on:** four exercises in `01-rust-fundamentals/` (move-vs-copy, string-vs-str,
borrow-rules, returning-references), a Phase 1 Cargo workspace, `scripts/check.sh`, and CI.
**Learned:** a move copies only the 24-byte `String` header, never the heap bytes; tests
comparing `as_ptr()` prove it. The `&`/`&mut` exclusion exists because `push` may reallocate
and leave a reference dangling (use-after-free).
**Stuck on:** `cargo new` inside an existing repo skips `.gitignore`, so `target/` almost got
committed. Running `cargo` from PowerShell fails: the toolchain lives in WSL, so use the
Ubuntu terminal (or tmux).
**Next:** #2, error handling: Rust Book ch. 9, custom error enum, `From` conversions.

## 2026-10-28 — Lab setup

**Worked on:** repository scaffold, MIT license, roadmap, and GitHub workflow.
**Learned:** keeping code on the WSL filesystem (not `/mnt/c`) for Linux tooling and build speed.
**Stuck on:** —
**Next:** Phase 1: read The Rust Book chapters 1–4 and start the file-inspector project.

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

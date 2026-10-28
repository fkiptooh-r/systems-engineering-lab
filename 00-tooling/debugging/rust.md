# Debugging Rust

## The feedback loop, fastest first

| Command | What it does | When |
|---|---|---|
| `cargo check` | Type-checks without producing a binary | Constantly while editing |
| `cargo clippy` | `check` plus ~800 lints for common mistakes | Before committing |
| `cargo test` | Builds and runs unit, integration, and doc tests | Before committing |
| `cargo test -p sum-numbers parse` | Only one crate, only tests whose name contains `parse` | Narrowing down a failure |
| `cargo test -- --nocapture` | Show `println!` output from passing tests | Debugging a test |
| `cargo run --release` | Optimised build | Any time you measure speed |
| `scripts/check.sh` | fmt + clippy `-D warnings` + tests for every workspace | Before every push; CI runs the same |

## Reading a compiler error

```text
error[E0502]: cannot borrow `scores` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let first = &scores[0];
  |                  ------ immutable borrow occurs here
4 |     scores.push(40);
  |     ^^^^^^^^^^^^^^^ mutable borrow occurs here
5 |     println!("{first}");
  |                ----- immutable borrow later used here
```

1. **The code** (`E0502`): `rustc --explain E0502` prints a full explanation with examples.
2. **The headline**: what rule was broken.
3. **The labels**: read them as a story, top to bottom: borrowed here → mutated here → used
   again here. The fix is almost always to break one link of the story.
4. **`help:` lines**: concrete suggestions, often a ready-made fix. Read them, but check they
   fit your intent (`clone()` silences E0382 but may hide a design problem).

Every error I hit in Phase 1 is recorded with its meaning in the folder READMEs under
`01-rust-fundamentals/`.

## Backtraces

A panic prints only the location by default. `RUST_BACKTRACE=1` prints the call stack:

```text
$ RUST_BACKTRACE=1 ./panic
thread 'main' panicked at /tmp/panic.rs:1:38:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
stack backtrace:
   ...
   3: <core::result::Result<i32, core::num::error::ParseIntError>>::unwrap
   4: panic::parse
             at /tmp/panic.rs:1:38
   5: panic::main
             at /tmp/panic.rs:2:28
```

Read from the top and skip the frames inside `core::` / `std::`: the first frame in **your**
code (`panic::parse`) is where to look, and the frame below it says who called it.

## Debug printing

```rust
dbg!(&report.errors);        // prints file:line, the expression, and its Debug value to stderr
eprintln!("{err:?}");        // Debug format of one value
eprintln!("{err:#?}");       // pretty, multi-line
```

`dbg!` returns its argument, so it can wrap an expression in place:
`let n = dbg!(text.parse::<i64>());`. Remove before committing (clippy can flag it).

## gdb (after `sudo apt install gdb`)

```bash
cargo build                                   # debug build has symbols
rust-gdb target/debug/sum-numbers
(gdb) break sum_numbers::parse_numbers
(gdb) run /tmp/bad.txt
(gdb) next / step / print numbers / backtrace / continue
```

## Pitfalls

- `cargo fmt -- --check` failing in `scripts/check.sh`: run `cargo fmt --all` to fix, then re-check.
- Timed a debug build: debug Rust can be 10–50× slower. Always `--release` before measuring.

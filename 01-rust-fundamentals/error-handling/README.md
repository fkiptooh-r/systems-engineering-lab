# Error handling

Issue: #2 · Reading: [The Rust Book ch. 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

| Exercise | Run | Shows |
|---|---|---|
| [`sum-numbers`](sum-numbers/) | `cargo run -p sum-numbers -- <file>` | `Result`, `?`, a custom error enum, `From`, error chains, exit codes |

Run from `01-rust-fundamentals/`. Tests: `cargo test -p sum-numbers` (7 unit tests in
`src/lib.rs`, 4 integration tests in `tests/cli.rs` that run the real binary).

## The problem

Sum the integers in a text file, one per line. Every way that can fail must be reported
clearly, and none of them may crash the program:

| Input | Output (stderr) | Exit code |
|---|---|---|
| valid file | `12` on stdout | 0 |
| a bad line | `error: line 3: "three" is not an integer`<br>`  caused by: invalid digit found in string` | 1 |
| missing file | `error: could not read input`<br>`  caused by: No such file or directory (os error 2)` | 1 |
| empty file | `error: input contains no numbers` | 1 |
| total > `i64::MAX` | `error: line 2: total overflows i64` | 1 |
| no argument | `usage: sum-numbers <file>` | 2 |

## Three versions of the same program

### 1. `unwrap()` everywhere

```rust
let text = fs::read_to_string("numbers.txt").unwrap();
let total: i64 = text.lines().map(|l| l.parse::<i64>().unwrap()).sum();
```

What actually happened:

```text
thread 'main' panicked at src/main.rs:5:60:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }

thread 'main' panicked at src/main.rs:4:50:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

The program dies, the message is written for the programmer, not the user, and it
doesn't say which line was bad.

### 2. `?` with `Box<dyn Error>`

First attempt: use `?` in `main` without changing its signature.

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option`
  |
3 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
4 |     let text = fs::read_to_string("numbers.txt")?;
  |                                                 ^ cannot use the `?` operator in a function that returns `()`
help: consider adding return type
  |
3 ~ fn main() -> Result<(), Box<dyn std::error::Error>> {
```

**What it meant:** `?` means "if this is `Err`, return it from the current function". A
function that returns `()` has nowhere to return an error to.

Following the hint works and nothing panics, but every error is now an opaque
`Box<dyn Error>`. Callers and tests can't ask "was this a missing file or a bad line?".
Fine for a quick script; not for a library.

### 3. A custom error enum (the final version)

```rust
pub enum SumError {
    Io(io::Error),
    Parse { line: usize, text: String, source: ParseIntError },
    Empty,
    Overflow { line: usize },
}
```

Each failure is a named case that tests can `match` on. The pieces:

- `impl Display`: the human-readable message.
- `impl Error` with `source()`: exposes the underlying cause, so `main` can print
  `caused by:` lines by walking the chain.
- `impl From<io::Error>`: lets `?` convert automatically. Without it:

```text
error[E0277]: `?` couldn't convert the error to `SumError`
   |
10 |     let text = fs::read_to_string(path)?;
   |                                        ^ the trait `From<std::io::Error>` is not implemented for `SumError`
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
```

**What it meant:** `?` is roughly `match x { Ok(v) => v, Err(e) => return Err(From::from(e)) }`.
The `From::from` call is the part that was missing.

### Why not `From<ParseIntError>`?

The issue suggested converting `ParseIntError` with `From` too. I chose not to: a
`ParseIntError` only says "invalid digit"; it doesn't know *which line* it came from. A
`From` impl has no access to that context, so `?` alone would lose it. Instead:

```rust
trimmed.parse::<i64>().map_err(|source| SumError::Parse { line: index + 1, text: trimmed.to_string(), source })?
```

Rule of thumb: use `From` when the conversion needs no extra information, `map_err` when
the error should carry context about where it happened.

## Edge cases the tests pin down

- Empty input is `Err(Empty)`, not `Ok(0)`: "no data" and "the data sums to zero" are
  different situations, and the caller should be able to tell.
- `99999999999999999999` fails in **parsing** (`number too large to fit in target type`).
- `i64::MAX + 1` parses fine but fails in **adding**: `checked_add` returns `None` instead
  of silently wrapping (release builds) or panicking (debug builds).

## When is `panic!` the right choice?

Panic means "this program has a bug", not "the input was bad".

- **Return `Result`** for anything the outside world can cause: missing files, bad input,
  network failures, permission errors. The caller decides what to do.
- **Panic** when an invariant *your own code* guarantees is broken, so continuing would
  be wrong: an index you just bounds-checked, a lock that cannot be poisoned in your
  design, impossible enum states.
- **`unwrap`/`expect` are fine** in tests, examples, and prototypes, and where failure is
  truly impossible. Prefer `expect("why this can't fail")` so the panic explains itself.
- In a library, panicking on bad input takes the decision away from the caller. In a
  long-running server, it can take down a worker thread or the whole process.

## Exit codes

`main` returns `std::process::ExitCode`: 0 success, 1 failure, 2 usage error (the shell
convention). Scripts and CI rely on this: `sum-numbers data.txt && echo ok` only prints
`ok` on success. Check with `echo $?` after running.

# Shell basics

## Three standard streams

Every process starts with three open file descriptors:

| fd | Name | Used for |
|---|---|---|
| 0 | stdin | input |
| 1 | stdout | results (data meant for other programs) |
| 2 | stderr | diagnostics (messages meant for a human) |

file-inspector and sum-numbers print results on stdout and errors on stderr, so
`file-inspector dir > hashes.txt` captures clean data while errors still reach the screen.

## Redirection

| Syntax | Meaning |
|---|---|
| `cmd > f` | stdout to file (overwrite) |
| `cmd >> f` | stdout to file (append) |
| `cmd 2> f` | stderr to file |
| `cmd > f 2>&1` | both to the same file (**order matters**: redirect stdout first, then point 2 at it) |
| `cmd 2>/dev/null` | discard errors |
| `cmd < f` | stdin from file |

Real example: one command, one success and one failure, split into two files:

```text
$ ls README.md /nope > /tmp/out.txt 2> /tmp/err.txt
$ cat /tmp/out.txt
README.md
$ cat /tmp/err.txt
ls: cannot access '/nope': No such file or directory
```

## Pipes

`a | b` connects a's stdout to b's stdin. stderr is **not** piped. To pipe both:
`a 2>&1 | b`.

```bash
cargo test 2>&1 | grep -E "test result|FAILED"    # cargo writes progress to stderr
```

## Exit codes

`$?` holds the exit code of the last command. `0` means success; anything else is failure.

```text
$ false; echo $?                         -> 1
$ grep -q nothing-matches README.md; echo $?   -> 1   (grep: 1 = no match, 2 = error)
$ ls /nope 2>/dev/null; echo $?          -> 2
```

Chain on them:

```bash
cargo test && git push            # push only if tests pass
file-inspector dir || echo "some files could not be read"
```

This is why file-inspector exits `1` when any file was unreadable: a script can tell
partial failure from success without parsing the output.

## Environment variables

```bash
echo "$HOME" "$PATH"              # read
export RUST_BACKTRACE=1           # set for this shell and every child process
RUST_LOG=debug cargo run          # set for one command only
env | sort                        # list all
```

Rust reads them with `std::env::var("NAME")`.

`PATH` is why `cargo` works in Ubuntu but not PowerShell: `~/.cargo/env`, sourced from
`~/.bashrc`, adds `~/.cargo/bin` to Ubuntu's `PATH`.

## Pitfalls

- `cmd 2>&1 > f` sends stderr to the *terminal*: `2>&1` copied where stdout pointed at that
  moment, before `> f` moved it. Write `> f 2>&1`.
- Variables inside single quotes are not expanded: `echo '$HOME'` prints `$HOME`.

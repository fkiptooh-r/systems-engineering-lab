# Searching: grep, find, xargs

All examples were run from the lab root.

## grep: search file contents

| Flag | Meaning |
|---|---|
| `-r` | recursive |
| `-n` | show line numbers |
| `-i` | ignore case |
| `-l` | only list matching file names |
| `-c` | count matches per file |
| `-E` | extended regex (`a|b`, `+`, `?`) |
| `-q` | quiet: no output, only the exit code |
| `--include='*.rs'` | only these files |

```text
$ grep -rn --include='*.rs' 'impl From' 01-rust-fundamentals
01-rust-fundamentals/error-handling/sum-numbers/src/lib.rs:57:impl From<io::Error> for SumError {

$ grep -rc --include='*.rs' '^\s*fn ' 01-rust-fundamentals | grep -v ':0$'
01-rust-fundamentals/cli-tools/file-inspector/tests/cli.rs:10
01-rust-fundamentals/cli-tools/file-inspector/src/lib.rs:10
01-rust-fundamentals/error-handling/sum-numbers/src/lib.rs:10
...
```

`rg` (ripgrep, written in Rust) does the same, faster, and skips `target/` and anything
in `.gitignore` automatically: `sudo apt install ripgrep`.

## find: search by name, type, size, time

```text
$ find 01-rust-fundamentals -name '*.rs' -not -path '*/target/*' | sort
01-rust-fundamentals/borrowing/borrow-rules/src/main.rs
01-rust-fundamentals/borrowing/returning-references/src/main.rs
01-rust-fundamentals/cli-tools/file-inspector/src/lib.rs
...
```

| Test | Meaning |
|---|---|
| `-name '*.rs'` | name matches (quote it, or the shell expands `*` first) |
| `-type f` / `-type d` / `-type l` | file / directory / symlink |
| `-size +100M` | bigger than 100 MiB |
| `-mtime -1` | modified in the last 24 hours |
| `-empty` | empty file or directory |
| `-not -path '*/target/*'` | exclude build output |

The lab's initial `.gitkeep` files were created with
`find . -type d -empty -exec touch {}/.gitkeep \;`.

## xargs: turn lines into arguments

```text
$ find 01-rust-fundamentals -name '*.rs' -not -path '*/target/*' -print0 | xargs -0 wc -l | tail -1
 1038 total
```

`-print0` / `-0` separate names with a NUL byte instead of a newline, so file names
containing spaces don't break apart. Always use the pair.

## Pitfalls

- Forgot `-not -path '*/target/*'` and got hundreds of generated files from `target/`.
- Unquoted `-name *.rs` worked in one folder and failed in another: the shell expanded
  `*.rs` before `find` ever saw it.

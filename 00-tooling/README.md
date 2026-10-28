# Tooling handbook

Issue: #5 · A personal reference, grown from real work in this lab. Each note is a short
cheat sheet: the commands, a real example from a project here, and the pitfalls to avoid.

| Note | Covers |
|---|---|
| [debugging/setup.md](debugging/setup.md) | Tool versions, installing debuggers, why the lab lives on the WSL filesystem |
| [shell/basics.md](shell/basics.md) | Pipes, redirection, exit codes, environment variables |
| [shell/search.md](shell/search.md) | `grep`, `find`, `xargs` |
| [linux/permissions.md](linux/permissions.md) | Reading `ls -l`, `chmod`, owners, groups, `umask` |
| [debugging/rust.md](debugging/rust.md) | `cargo check`/`clippy`/`test`, backtraces, reading compiler errors |
| [debugging/strace.md](debugging/strace.md) | Watching system calls: the `chmod 000` case from file-inspector |

Add to these whenever something takes more than ten minutes to figure out.

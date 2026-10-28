# Environment setup

## Versions (2026-10-28)

```text
$ uname -r          5.15.167.4-microsoft-standard-WSL2
$ rustc --version   rustc 1.99.1 (58cf8f9c3 2026-10-13)
$ cargo --version   cargo 1.99.1 (5f94df478 2026-08-27)
$ strace -V         strace -- version 6.8
$ git --version     git version 2.43.0
```

Ubuntu 24.04 LTS on WSL2, 4 CPUs, ~10 GiB RAM.

## Install the debugging tools

```bash
sudo apt update
sudo apt install gdb strace valgrind
```

| Tool | Answers |
|---|---|
| `strace` | Which system calls does my program make, with what arguments, and what did the kernel return? |
| `gdb` | Where exactly is the program, and what's in its variables right now? (`rust-gdb` adds Rust pretty-printing) |
| `valgrind` | Is memory being leaked or misused? (Mostly for C; Rust rarely needs it, but `unsafe` code does) |
| `/usr/bin/time -v` | Peak memory (max RSS) and CPU time. Not the shell's `time` builtin, which only shows time. |

**No sudo?** Unpack a package into a private folder without installing it:

```bash
mkdir -p ~/.cache/tools && cd ~/.cache/tools
apt-get download strace && dpkg -x strace_*.deb root
~/.cache/tools/root/usr/bin/strace -V
```

(This is how the examples in [strace.md](strace.md) were first captured.)

## Why the lab lives in `~/projects`, not `/mnt/c`

WSL2 has two filesystems:

- `~/...`: a real Linux ext4 filesystem inside the WSL virtual machine. Fast, and Linux
  permissions, symlinks, and inotify all work.
- `/mnt/c/...`: the Windows drive, reached through a translation layer (9P). Every file
  operation crosses the VM boundary, so builds that touch thousands of files are many times
  slower, and `chmod` doesn't behave like real Linux.

Rust builds and permission experiments (`chmod 000` in file-inspector's tests) need the
real Linux filesystem. Windows tools can still reach it at
`\\wsl.localhost\Ubuntu\home\flex\projects\systems-engineering-lab`.

## Mistakes I made

- Running `cargo` from PowerShell: Rust is installed **inside** Ubuntu, so Windows can't see
  it. Use the Ubuntu terminal, or `wsl -- bash -lc "cd ~/projects/systems-engineering-lab && cargo test"`.
- `cargo new` inside an existing git repo doesn't create a `.gitignore`, so `target/` nearly
  got committed. Fixed with a root `.gitignore`.

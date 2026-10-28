# File permissions

## Reading `ls -l`

```text
-rw-r----- 1 flex flex 0 Oct 28 13:19 f
│└┬┘└┬┘└┬┘   └┬─┘ └┬─┘
│ │  │  │     │    └─ group
│ │  │  │     └────── owner
│ │  │  └─ other: ---  (no access)
│ │  └──── group: r--  (read)
│ └─────── owner: rw-  (read, write)
└───────── type: - file, d directory, l symlink
```

## What r, w, x mean

| | File | Directory |
|---|---|---|
| `r` | read contents | list names (`ls`) |
| `w` | change contents | create, delete, rename entries inside |
| `x` | execute | enter / access entries by name (`cd`, open files inside) |

Deleting a file needs `w` on the **directory**, not on the file.

## chmod

Numeric: each digit is owner, group, other; `r=4 w=2 x=1`, added together.

| Mode | Meaning | Typical use |
|---|---|---|
| `644` | `rw-r--r--` | normal files |
| `755` | `rwxr-xr-x` | programs, directories |
| `600` | `rw-------` | private keys, secrets |
| `000` | `---------` | nothing (used to test error handling) |

Symbolic: `chmod u+x,o+r f` adds execute for the owner and read for others.

```text
$ chmod 640 f && stat -c '%a %A %U:%G' f
640 -rw-r----- flex:flex
$ chmod u+x,o+r f && stat -c '%a %A' f
744 -rwxr--r--
```

## umask

New files start at `666` (dirs `777`) minus the umask. Mine is `0022`, so new files are
`644` and new directories `755`.

## Users and groups

```text
$ id
uid=1000(flex) gid=1000(flex) groups=1000(flex),4(adm),27(sudo),...,1001(docker)
```

Being in `docker` is why `docker` works without `sudo`; being in `sudo` allows `sudo`.

## Root ignores permission bits

`uid 0` bypasses read/write checks. file-inspector's `chmod 000` tests therefore check
whether the file is still openable and skip if so, instead of failing when run as root
(as some CI containers do).

## In Rust

```rust
use std::os::unix::fs::PermissionsExt;
fs::set_permissions(&path, fs::Permissions::from_mode(0o000))?;   // note the 0o: octal
```

A denied open becomes `io::Error` with `kind() == PermissionDenied`, printed as
`Permission denied (os error 13)`. 13 is `EACCES`; see [strace.md](../debugging/strace.md).

## Pitfalls

- Wrote `from_mode(644)` (decimal) instead of `0o644` (octal): 644 decimal is `0o1204`,
  a completely different mode.

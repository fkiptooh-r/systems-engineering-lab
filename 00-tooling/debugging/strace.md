# strace: watching system calls

A program cannot open a file, read a directory, or print text by itself; it asks the
kernel through **system calls**. `strace` shows every one, with its arguments and the
kernel's answer. When a program fails mysteriously, strace shows what the kernel actually
said.

## Essential commands

| Command | Shows |
|---|---|
| `strace ./prog` | every syscall |
| `strace -e trace=openat,read ./prog` | only these calls |
| `strace -e trace=file ./prog` | every call that takes a path |
| `strace -f ./prog` | include child processes and threads |
| `strace -c ./prog` | a summary table: count, errors, time per syscall |
| `strace -o trace.txt ./prog` | write the trace to a file |
| `strace -p <pid>` | attach to an already running process |

strace writes to **stderr**: `strace ./prog 2>&1 >/dev/null | grep EACCES` drops the
program's output and keeps the trace.

## The `chmod 000` case from file-inspector (#3)

```text
$ ls -l /tmp/strace-demo
-rw-r--r-- 1 flex flex  3 Oct 28 13:15 ok.txt
---------- 1 flex flex 10 Oct 28 13:15 secret.txt

$ file-inspector /tmp/strace-demo; echo "exit=$?"
           3  ba7816bf...0015ad  /tmp/strace-demo/ok.txt
error: cannot read file /tmp/strace-demo/secret.txt: Permission denied (os error 13)
1 files, 3 bytes, 1 errors
exit=1
```

The same run under strace (`-e trace=openat,statx,getdents64`, arguments shortened):

```text
statx(AT_FDCWD, "/tmp/strace-demo", AT_SYMLINK_NOFOLLOW, ..., {stx_mode=S_IFDIR|0755, ...}) = 0
openat(AT_FDCWD, "/tmp/strace-demo", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = 3
getdents64(3, /* 4 entries */, 32768) = 112
getdents64(3, /* 0 entries */, 32768) = 0
statx(AT_FDCWD, "/tmp/strace-demo/ok.txt", AT_SYMLINK_NOFOLLOW, ..., {stx_mode=S_IFREG|0644, stx_size=3, ...}) = 0
openat(AT_FDCWD, "/tmp/strace-demo/ok.txt", O_RDONLY|O_CLOEXEC) = 3
statx(AT_FDCWD, "/tmp/strace-demo/secret.txt", AT_SYMLINK_NOFOLLOW, ..., {stx_mode=S_IFREG|000, stx_size=10, ...}) = 0
openat(AT_FDCWD, "/tmp/strace-demo/secret.txt", O_RDONLY|O_CLOEXEC) = -1 EACCES (Permission denied)
```

Mapping each line back to the Rust code:

| Syscall | Rust code | Note |
|---|---|---|
| `statx(..., AT_SYMLINK_NOFOLLOW)` | `fs::symlink_metadata(&path)` | NOFOLLOW is the `lstat` behaviour that makes symlink loops impossible |
| `openat(... O_DIRECTORY) = 3` | `fs::read_dir(&path)` | returns file descriptor 3 (0–2 are stdin/out/err) |
| `getdents64` | iterating `ReadDir` | 4 entries = `.`, `..`, and the two files; the second call returning 0 means "end" |
| `openat(...secret.txt) = -1 EACCES` | `File::open` in `hash_file` | the kernel's answer |

`EACCES` is error number **13**. Rust wraps it as `io::Error` with kind `PermissionDenied`,
and its `Display` prints `Permission denied (os error 13)`: the exact text in the error
message. **The `stat` of secret.txt succeeded** (mode `000` is visible) and only the `open`
failed: reading metadata needs permission on the *directory*, reading contents needs
permission on the *file*.

## Proving the chunked reads

A 150 000-byte file, `strace -e trace=read`:

```text
read(3, ..., 832) = 832          <- the dynamic loader reading shared libraries
read(3, ..., 1024) = 1024           (fd 3 is reused; ignore these)
read(3, ..., 65536) = 65536      <- hash_reader: one 64 KiB buffer
read(3, ..., 65536) = 65536
read(3, ..., 65536) = 18928      <- partial read: 65536 + 65536 + 18928 = 150000
read(3, ..., 65536) = 0          <- 0 bytes = end of file; the loop's `Ok(0) => break`
```

The buffer size from `CHUNK_SIZE`, the partial final read, and the `0` that ends the loop
are all visible. This is why memory stays at ~3 MB even for a 1 GiB file.

## Summary mode

`strace -c file-inspector /tmp/strace-demo` (top rows):

```text
% time     seconds  usecs/call     calls    errors syscall
 21.64    0.001386          81        17           write
 16.20    0.001038          79        13           mmap
  7.48    0.000479          68         7         1 openat
  2.51    0.000161          53         3           statx
  2.20    0.000141          70         2           getdents64
```

The `errors` column catches failures at a glance: 1 failed `openat`, the `EACCES`.

## Pitfalls

- Piped `strace ./prog | grep` and saw nothing: strace writes to stderr. Use `2>&1`.
- Expected `stat`/`lstat` in the output; modern glibc uses `statx` (and `newfstatat`). Trace
  `-e trace=file` instead of guessing names.

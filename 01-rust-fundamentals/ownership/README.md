# Ownership

Issue: #1 · Reading: [The Rust Book ch. 4.1–4.3](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

| Exercise | Run | Shows |
|---|---|---|
| [`move-vs-copy`](move-vs-copy/src/main.rs) | `cargo run -p move-vs-copy` | `Copy` types vs moves vs `clone()` |
| [`string-vs-str`](string-vs-str/src/main.rs) | `cargo run -p string-vs-str` | Owned `String` vs borrowed `&str` |

Run from `01-rust-fundamentals/`. Tests: `cargo test -p <name>`.

## The rules

1. Every value has exactly one owner.
2. When the owner goes out of scope, the value is dropped (heap memory freed).
3. Assigning or passing a non-`Copy` value **moves** it: the old name can no longer be used.

Rule 3 exists because of rule 2: if two variables owned the same heap buffer, both would
free it at the end of scope (a *double free*). C leaves that to the programmer; Rust makes
it a compile error.

## Errors I hit

### E0382: borrow of moved value

```text
error[E0382]: borrow of moved value: `s1`
  |
6 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
7 |     let s2 = s1;
  |              -- value moved here
8 |     println!("{s1} {s2}");
  |                ^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
7 |     let s2 = s1.clone();
```

**What it meant:** `let s2 = s1` copied only the 24-byte header (pointer, length, capacity)
and made `s2` the owner of the heap bytes. `s1` is now invalid, so reading it is refused.
The same error appears after passing `s3` to `takes_ownership(s: String)`: the function
became the owner and freed the buffer when it returned.

**Fixes, in order of preference:**
- Don't need ownership? Borrow: change the parameter to `&str` / `&String` and pass `&s3`.
- Need ownership back? Return it from the function (`gives_back`).
- Really need two independent copies? `clone()`, and accept the allocation cost.

### E0308: expected `&String`, found `&str`

```text
error[E0308]: mismatched types
  |
6 |     println!("{}", shout("hello"));
  |                    ----- ^^^^^^^ expected `&String`, found `&str`
  |
  = note: expected reference `&String`
             found reference `&'static str`
```

**What it meant:** a string literal is a `&'static str` pointing into the program binary;
it is not a `String`, so it cannot be passed where `&String` is required. Going the other
way works automatically: `&String` coerces to `&str`. So read-only functions should take
`&str`, which accepts both.

## Systems view

Proved in the tests, not just claimed:

- A move does **not** copy heap data: `s1.as_ptr() == s2.as_ptr()` after `let s2 = s1`.
- `clone()` allocates a **new** buffer: the pointers differ.
- `String` is 3 machine words (24 bytes on x86-64); `&str` is 2 (pointer + length).
- `first_word(&owned)` returns a slice that points *into* `owned`'s buffer: zero copies.

## Questions to come back to

- What exactly happens in `drop` for a `String`? (Phase 3: allocators)
- When is `clone()` cheap? (`Rc`/`Arc` clone a pointer and bump a counter.)

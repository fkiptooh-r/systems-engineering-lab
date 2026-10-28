# Borrowing and lifetimes

Issue: #1 · Reading: [The Rust Book ch. 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html), [ch. 10.3](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

| Exercise | Run | Shows |
|---|---|---|
| [`borrow-rules`](borrow-rules/src/main.rs) | `cargo run -p borrow-rules` | Shared vs mutable borrows, and why they conflict |
| [`returning-references`](returning-references/src/main.rs) | `cargo run -p returning-references` | Lifetime annotations vs returning owned values |

Run from `01-rust-fundamentals/`. Tests: `cargo test -p <name>`.

## The rules

- Any number of shared references `&T`, **or** exactly one mutable reference `&mut T`, never both.
- A reference must never outlive the value it points to.
- A borrow lasts until its **last use**, not until the end of the block (non-lexical lifetimes).

## Errors I hit

### E0502: cannot borrow as mutable because it is also borrowed as immutable

```text
error[E0502]: cannot borrow `scores` as mutable because it is also borrowed as immutable
  |
3 |     let first = &scores[0];
  |                  ------ immutable borrow occurs here
4 |     scores.push(40);
  |     ^^^^^^^^^^^^^^^ mutable borrow occurs here
5 |     println!("{first}");
  |                ----- immutable borrow later used here
```

**What it meant:** `push` may need more capacity, which reallocates the buffer, and the
allocator may move it. `first` would then point at freed memory: a *use-after-free*,
one of the most common C bugs. Rust rejects it at compile time.

**Fix:** finish using `first` before calling `push` (the borrow then ends), or copy the
value out (`let first = scores[0];`) instead of borrowing it.

**Observed:** in my run, growing from capacity 1 did *not* move the buffer (the allocator
extended it in place). That is luck, not a guarantee, which is exactly why the compiler
cannot allow the borrow.

### E0106: missing lifetime specifier

```text
error[E0106]: missing lifetime specifier
  |
1 | fn longest(a: &str, b: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature
          does not say whether it is borrowed from `a` or `b`
help: consider introducing a named lifetime parameter
  |
1 | fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
```

**What it meant:** callers need to know how long the result stays valid, and that depends
on which input it borrows from. `'a` says "the result is valid only while **both** inputs
are". Annotations don't extend any lifetime; they describe the relationship so the
compiler can check each call site.

The same error code, for `fn dangle() -> &String`, has a different cause:

```text
  = help: this function's return type contains a borrowed value, but there is no value
          for it to be borrowed from
help: instead, you are more likely to want to return an owned value
  |
5 + fn dangle() -> String {
```

**What it meant:** the `String` created inside the function is dropped when the function
returns, so there is nothing left to borrow. No annotation can fix that; return the
owned `String`.

### E0597: does not live long enough

```text
error[E0597]: `inner` does not live long enough
   |
 9 |         let inner = String::from("short");
   |             ----- binding `inner` declared here
10 |         result = longest(outer.as_str(), inner.as_str());
   |                                          ^^^^^ borrowed value does not live long enough
11 |     }
   |     - `inner` dropped here while still borrowed
12 |     println!("{result}");
   |                ------ borrow later used here
```

**What it meant:** this is `'a` doing its job. `result` might borrow from `inner`, and
`inner` is dropped at the `}` while `result` is still used afterwards.

**Two fixes, both in the code:**
1. Use the borrowed result inside the scope where both inputs are alive.
2. Return an owned `String` (`longest_owned`) so the result doesn't depend on the inputs.

## Takeaways

- The borrow checker prevents use-after-free and data races at compile time, at zero runtime cost.
- When fighting a lifetime error, ask "who owns this data, and does it outlive the reference?"
  before reaching for annotations.
- Returning owned data is often the simplest correct answer; borrow when profiling says the
  copy matters.

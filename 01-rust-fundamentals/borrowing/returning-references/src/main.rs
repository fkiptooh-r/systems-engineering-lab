//! Returning references and lifetimes.
//!
//! A returned reference must point at data that outlives the call. Lifetime
//! annotations do not change how long anything lives; they tell the compiler
//! which input the output borrows from, so it can check every caller.
//! Uncomment a `// BROKEN:` block to reproduce the errors recorded in
//! ../README.md.

// BROKEN: E0106 missing lifetime specifier: is the result borrowed from `a` or `b`?
// fn longest(a: &str, b: &str) -> &str { if a.len() >= b.len() { a } else { b } }

// BROKEN: E0106 again: the String is dropped at the end of the function, so
// there is nothing valid left to borrow from.
// fn dangle() -> &String { let s = String::from("temporary"); &s }

/// Fix 1: annotate. The result lives no longer than the shorter-lived input.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

/// Fix 2: return an owned value. The caller takes ownership; no lifetimes.
fn longest_owned(a: &str, b: &str) -> String {
    longest(a, b).to_string()
}

/// Fix for `dangle`: return the String itself instead of a reference to it.
fn no_dangle() -> String {
    String::from("temporary")
}

fn main() {
    let outer = String::from("long string");

    {
        let inner = String::from("short");
        let result = longest(&outer, &inner);
        // Fine: `result` is used while both `outer` and `inner` are alive.
        println!("longest = {result}");
    }

    let owned;
    {
        let inner = String::from("short");
        // BROKEN: `borrowed = longest(&outer, &inner);` declared outside this
        // block and printed after it gives E0597: `inner` does not live long
        // enough. An owned result has no such restriction:
        owned = longest_owned(&outer, &inner);
    }
    println!("owned = {owned}");

    println!("no_dangle = {}", no_dangle());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_borrows_from_the_longer_input() {
        let a = String::from("abc");
        let b = String::from("abcdef");
        let result = longest(&a, &b);
        assert_eq!(result, "abcdef");
        assert_eq!(result.as_ptr(), b.as_ptr());
    }

    #[test]
    fn ties_return_the_first_argument() {
        let a = "one";
        let b = "two";
        assert_eq!(longest(a, b).as_ptr(), a.as_ptr());
    }

    #[test]
    fn owned_result_outlives_its_inputs() {
        let result;
        {
            let a = String::from("short");
            let b = String::from("much longer");
            result = longest_owned(&a, &b);
        }
        assert_eq!(result, "much longer");
    }

    #[test]
    fn owned_return_replaces_dangling_reference() {
        assert_eq!(no_dangle(), "temporary");
    }
}

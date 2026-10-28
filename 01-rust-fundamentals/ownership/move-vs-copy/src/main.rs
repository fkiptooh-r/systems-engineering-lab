//! Move vs copy.
//!
//! `i32` is `Copy`: assignment duplicates the value and both variables stay usable.
//! `String` owns a heap buffer: assignment *moves* ownership and the old variable
//! becomes unusable. Uncomment a `// BROKEN:` line to reproduce the error
//! recorded in ../README.md.

fn main() {
    // Copy: an i32 is 4 bytes on the stack; copying it is cheap and implicit.
    let a: i32 = 42;
    let b = a;
    println!("a = {a}, b = {b}");

    // Move: a String is (pointer, length, capacity) on the stack plus bytes on
    // the heap. Assignment copies the three stack words and invalidates `s1`,
    // so exactly one owner will free the heap buffer.
    let s1 = String::from("hello");
    let s2 = s1;
    // BROKEN: println!("{s1}"); // E0382: borrow of moved value: `s1`
    println!("s2 = {s2}");

    // Clone: an explicit, visible deep copy with its own heap buffer.
    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}");

    // Passing by value moves into the function, which drops it on return.
    takes_ownership(s3);
    // BROKEN: println!("{s3}"); // E0382: `s3` was moved into the function

    // Returning a value hands ownership back to the caller.
    let s4 = gives_back(String::from("returned"));
    println!("s4 = {s4}");
}

fn takes_ownership(s: String) {
    println!("took {s}; its heap buffer is freed when this function returns");
}

fn gives_back(s: String) -> String {
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn copy_leaves_both_values_usable() {
        let a = 7;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn move_transfers_the_same_heap_buffer() {
        let s1 = String::from("hello");
        let before = s1.as_ptr();
        let s2 = s1;
        // No bytes were copied: the new owner points at the same allocation.
        assert_eq!(before, s2.as_ptr());
    }

    #[test]
    fn clone_allocates_a_new_heap_buffer() {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        assert_eq!(s1, s2);
        assert_ne!(s1.as_ptr(), s2.as_ptr());
    }

    #[test]
    fn ownership_round_trips_through_a_function() {
        let s = String::from("data");
        let ptr = s.as_ptr();
        let back = super::gives_back(s);
        assert_eq!(ptr, back.as_ptr());
    }
}

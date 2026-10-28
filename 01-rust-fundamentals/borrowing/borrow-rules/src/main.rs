//! The borrowing rules: at any moment a value may have *either* any number of
//! shared references (`&T`) *or* exactly one mutable reference (`&mut T`).
//!
//! The rule is not arbitrary: pushing to a `Vec` can reallocate its buffer,
//! which would leave an existing `&` reference pointing at freed memory.
//! Uncomment the `// BROKEN:` line to reproduce the error recorded in
//! ../README.md.

/// Reads through a shared borrow; the caller keeps ownership.
fn total(scores: &[i32]) -> i32 {
    scores.iter().sum()
}

/// Mutates through an exclusive borrow; the caller keeps ownership.
fn add_bonus(scores: &mut [i32], bonus: i32) {
    for score in scores.iter_mut() {
        *score += bonus;
    }
}

fn main() {
    let mut scores = vec![10, 20, 30];

    let first = &scores[0];
    // BROKEN: scores.push(40); // E0502: cannot borrow `scores` as mutable
    //                          // because it is also borrowed as immutable
    println!("first = {first}");

    // Fine: `first` is never used again, so its borrow has already ended
    // (non-lexical lifetimes).
    scores.push(40);

    // Many shared borrows at once are fine.
    let a = &scores;
    let b = &scores;
    println!("total via a = {}, len via b = {}", total(a), b.len());

    add_bonus(&mut scores, 5);
    println!("after bonus = {scores:?}");

    // Why the rule exists: growing past capacity reallocates, and the
    // allocator *may* move the buffer. Whether it moves this run or not, the
    // compiler must assume it can, so a live `&v[0]` is never allowed here.
    let mut v = Vec::with_capacity(1);
    v.push(1);
    let before = v.as_ptr();
    v.push(2);
    println!(
        "capacity 1 -> {}, buffer address changed: {}",
        v.capacity(),
        before != v.as_ptr()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_borrow_reads_without_taking_ownership() {
        let scores = vec![1, 2, 3];
        assert_eq!(total(&scores), 6);
        assert_eq!(scores.len(), 3); // still usable
    }

    #[test]
    fn mutable_borrow_changes_the_original() {
        let mut scores = vec![1, 2, 3];
        add_bonus(&mut scores, 10);
        assert_eq!(scores, vec![11, 12, 13]);
    }

    #[test]
    fn borrow_ends_after_last_use() {
        let mut scores = vec![1];
        let first = scores[0]; // copy out the i32, not a reference
        scores.push(2);
        assert_eq!(first, 1);
        assert_eq!(scores, vec![1, 2]);
    }
}

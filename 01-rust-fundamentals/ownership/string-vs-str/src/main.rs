//! `String` vs `&str`.
//!
//! `String` owns a growable heap buffer. `&str` is a borrowed view into UTF-8
//! bytes that live somewhere else: inside a `String`, or in the program binary
//! for a literal. A function that only reads text should take `&str`, so it
//! accepts both. Uncomment the `// BROKEN:` line to reproduce the error
//! recorded in ../README.md.

/// Returns the first whitespace-separated word, borrowed from the input.
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// BROKEN: taking `&String` rejects string literals.
// fn first_word_strict(s: &String) -> &str { ... }
// first_word_strict("hello") // E0308: expected `&String`, found `&str`

fn main() {
    let owned: String = String::from("hello world");
    let literal: &str = "systems engineering";

    // `&String` coerces to `&str` automatically (deref coercion).
    println!("{}", first_word(&owned));
    // A literal is already a `&'static str`, stored in the binary.
    println!("{}", first_word(literal));
    // A slice of a String is also a `&str`; no bytes are copied.
    println!("{}", first_word(&owned[6..]));

    // Sizes on a 64-bit target: String = ptr + len + capacity; &str = ptr + len.
    println!(
        "size_of String = {} bytes, size_of &str = {} bytes",
        std::mem::size_of::<String>(),
        std::mem::size_of::<&str>()
    );
}

#[cfg(test)]
mod tests {
    use super::first_word;

    #[test]
    fn accepts_string_and_literal() {
        let owned = String::from("rust is fun");
        assert_eq!(first_word(&owned), "rust");
        assert_eq!(first_word("linux rocks"), "linux");
    }

    #[test]
    fn handles_empty_and_leading_whitespace() {
        assert_eq!(first_word(""), "");
        assert_eq!(first_word("   padded  "), "padded");
    }

    #[test]
    fn result_borrows_from_the_input_without_copying() {
        let owned = String::from("hello world");
        let word = first_word(&owned);
        // The returned slice points into the original heap buffer.
        assert_eq!(word.as_ptr(), owned.as_ptr());
    }

    #[test]
    fn string_is_three_words_str_is_two() {
        let word = std::mem::size_of::<usize>();
        assert_eq!(std::mem::size_of::<String>(), 3 * word);
        assert_eq!(std::mem::size_of::<&str>(), 2 * word);
    }
}

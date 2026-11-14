/// Borrow `name` and return a new greeting.
///
/// The caller keeps the original text. This function only reads it.
///
/// ```
/// assert_eq!(greet::greet("Ada"), "Hello, Ada!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Grow one greeting by editing the same `String`.
///
/// `greeting` is `mut` because each `push` changes it. Without `mut` this
/// function does not compile.
///
/// ```
/// assert_eq!(greet::excited("Ada"), "Hello, Ada!");
/// ```
pub fn excited(name: &str) -> String {
    let mut greeting = String::from("Hello, ");
    greeting.push_str(name);
    greeting.push('!');
    greeting
}

/// Greet trimmed, lowercased text.
///
/// The second `let name` is a new binding, not a change to the first. The
/// first `name` is `&str`. The second is an owned `String`. This is
/// shadowing, which does not need `mut`.
///
/// ```
/// assert_eq!(greet::quiet_greet("  Ada  "), "Hello, ada!");
/// ```
pub fn quiet_greet(raw: &str) -> String {
    let name = raw.trim();
    let name = name.to_lowercase();
    format!("Hello, {name}!")
}

/// Shout `name` by taking ownership of it.
///
/// The parameter is `String`, not `&str`. The call moves `name`. Using `name`
/// after `into_shout(name)` does not compile.
///
/// ```
/// let name = String::from("ada");
/// assert_eq!(greet::into_shout(name), "ADA!");
/// ```
pub fn into_shout(name: String) -> String {
    format!("{}!", name.to_uppercase())
}

/// Return `count` twice. Both bindings stay usable.
///
/// `i32` is `Copy`, so `let saved = count` duplicates the value. A `String`
/// is not `Copy`. Passing one to `into_shout` moves it instead.
///
/// ```
/// assert_eq!(greet::copied_count(3), (3, 3));
/// ```
pub fn copied_count(count: i32) -> (i32, i32) {
    let saved = count;
    (count, saved)
}

/// Append `!` to the caller's string.
///
/// `&mut String` borrows `greeting` and allows this function to change it.
/// The caller still owns the `String`. `into_shout` would have taken it away.
///
/// ```
/// let mut greeting = String::from("Hello, Ada");
/// greet::push_bang(&mut greeting);
/// assert_eq!(greeting, "Hello, Ada!");
/// ```
pub fn push_bang(greeting: &mut String) {
    greeting.push('!');
}

/// Read the length, then append `!`.
///
/// `len()` borrows `greeting` to read it. That shared borrow ends at the
/// semicolon. `push` then borrows it mutably. A shared borrow and a mutable
/// borrow cannot both be active.
///
/// This does not compile, because `seen` still borrows `greeting` during `push`:
///
/// ```text
/// let seen = greeting.as_str();
/// greeting.push('!');
/// let _ = seen;
/// ```
pub fn len_then_bang(greeting: &mut String) -> usize {
    let len = greeting.len();
    greeting.push('!');
    len
}

/// Return the first word, or `""` when `text` has none.
///
/// The result is a `&str` pointing into `text`. No new `String` is allocated.
/// Returning `owned.as_str()` from a local `String` does not compile: that
/// `String` is dropped at the end of the function, so the borrow would dangle.
///
/// ```
/// assert_eq!(greet::first_word("Hello Ada"), "Hello");
/// ```
pub fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

/// The first word, if `text` has one.
///
/// `Some(word)` borrows that word from `text`. `None` means there is no word.
/// `first_word` turns that `None` into `""`. This function returns the
/// `Option` itself, so the caller has to handle both cases.
///
/// ```
/// assert_eq!(greet::maybe_first_word("Hello Ada"), Some("Hello"));
/// assert_eq!(greet::maybe_first_word("   "), None);
/// ```
pub fn maybe_first_word(text: &str) -> Option<&str> {
    text.split_whitespace().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caller_keeps_the_name() {
        let name = String::from("Ada");
        let greeting = greet(&name);
        assert_eq!(greeting, "Hello, Ada!");
        assert_eq!(name, "Ada");
    }

    #[test]
    fn excited_changes_one_string() {
        assert_eq!(excited("Ada"), greet("Ada"));
    }

    #[test]
    fn quiet_greet_shadows_a_trimmed_borrow_with_a_string() {
        assert_eq!(quiet_greet("  Ada  "), "Hello, ada!");
    }

    #[test]
    fn into_shout_takes_the_string() {
        let name = String::from("ada");
        let shouted = into_shout(name);
        assert_eq!(shouted, "ADA!");
    }

    #[test]
    fn copied_count_keeps_the_original() {
        let count = 3;
        let (left, right) = copied_count(count);
        assert_eq!((left, right), (3, 3));
        assert_eq!(count, 3);
    }

    #[test]
    fn push_bang_changes_the_callers_string() {
        let mut greeting = String::from("Hello, Ada");
        push_bang(&mut greeting);
        assert_eq!(greeting, "Hello, Ada!");
    }

    #[test]
    fn len_then_bang_reads_before_it_writes() {
        let mut greeting = String::from("Hello");
        let len = len_then_bang(&mut greeting);
        assert_eq!(len, 5);
        assert_eq!(greeting, "Hello!");
    }

    #[test]
    fn first_word_borrows_part_of_the_text() {
        let text = String::from("Hello Ada");
        let word = first_word(&text);
        assert_eq!(word, "Hello");
        assert_eq!(text, "Hello Ada");
        assert_eq!(first_word("   "), "");
    }

    #[test]
    fn maybe_first_word_is_some_or_none() {
        assert_eq!(maybe_first_word("Hello Ada"), Some("Hello"));
        assert_eq!(maybe_first_word("   "), None);
        assert_eq!(maybe_first_word(""), None);
    }
}

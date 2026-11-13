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
}

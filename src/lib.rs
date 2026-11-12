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
}

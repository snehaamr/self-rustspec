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
}

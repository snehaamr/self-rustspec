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

/// Greet the first word, or greet nobody when there is no word.
///
/// `match` has to list every variant. `Some(word)` names the borrowed word.
/// `None` is the other case. Deleting the `None` arm does not compile.
///
/// ```
/// assert_eq!(greet::greet_first_word("Ada Lovelace"), "Hello, Ada!");
/// assert_eq!(greet::greet_first_word("   "), "Hello!");
/// ```
pub fn greet_first_word(text: &str) -> String {
    match maybe_first_word(text) {
        Some(word) => format!("Hello, {word}!"),
        None => String::from("Hello!"),
    }
}

/// Greet the first word. When there is no word, return `text` unchanged.
///
/// `if let` handles one variant. The `else` branch covers everything else,
/// here `None`, without naming it. `greet_first_word` uses `match` and lists
/// both variants.
///
/// ```
/// assert_eq!(greet::greet_if_some("Ada Lovelace"), "Hello, Ada!");
/// assert_eq!(greet::greet_if_some("   "), "   ");
/// ```
pub fn greet_if_some(text: &str) -> String {
    if let Some(word) = maybe_first_word(text) {
        format!("Hello, {word}!")
    } else {
        text.to_string()
    }
}

/// A greeting. `Hello` carries a name. `Wave` carries nothing.
///
/// `Hello` holds a `String`, so `Greeting` is not `Copy`. `Option` is an enum
/// from the standard library. This one is defined here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Greeting {
    Hello(String),
    Wave,
}

/// Describe `greeting` without taking ownership of it.
///
/// The match lists every variant. `Hello(name)` binds the carried `String`
/// as a borrow, because the input is `&Greeting`.
///
/// ```
/// let greeting = greet::Greeting::Hello(String::from("Ada"));
/// assert_eq!(greet::describe(&greeting), "Hello, Ada!");
/// assert_eq!(greet::describe(&greet::Greeting::Wave), "wave");
/// ```
pub fn describe(greeting: &Greeting) -> String {
    match greeting {
        Greeting::Hello(name) => format!("Hello, {name}!"),
        Greeting::Wave => String::from("wave"),
    }
}

/// A person being greeted. Every `Person` has a `name`.
///
/// A struct keeps all of its fields at once. An enum is one variant at a time.
/// `name` is a `String`, so `Person` is not `Copy`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub name: String,
}

impl Person {
    /// Build a person from a borrowed name.
    ///
    /// `new` has no `self`, so it is an associated function, not a method.
    /// Call it as `Person::new`. It creates a `Person` instead of borrowing
    /// or moving an existing one.
    ///
    /// ```
    /// let ada = greet::Person::new("Ada");
    /// assert_eq!(ada.greeting(), "Hello, Ada!");
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    /// Build a person when `name` is not blank. Otherwise, say why it failed.
    ///
    /// `Ok(person)` carries the new `Person`. `Err` carries a message and no
    /// person. `new` always returns a `Person`, even for `""`. `Option` would
    /// only say the name was missing. `Result` also says what went wrong.
    ///
    /// ```
    /// let ada = greet::Person::try_new("Ada").unwrap();
    /// assert_eq!(ada.greeting(), "Hello, Ada!");
    /// assert_eq!(greet::Person::try_new("   "), Err("name is empty"));
    /// ```
    pub fn try_new(name: &str) -> Result<Self, &'static str> {
        let name = name.trim();
        if name.is_empty() {
            Err("name is empty")
        } else {
            Ok(Self::new(name))
        }
    }

    /// Borrow this person and return a greeting.
    ///
    /// `&self` is the same kind of shared borrow as `&Person`. The caller
    /// keeps the `Person`.
    ///
    /// ```
    /// let ada = greet::Person { name: String::from("Ada") };
    /// assert_eq!(ada.greeting(), "Hello, Ada!");
    /// ```
    pub fn greeting(&self) -> String {
        format!("Hello, {}!", self.name)
    }

    /// Replace this person's name. The caller still owns the `Person`.
    ///
    /// `&mut self` is a mutable borrow, the same kind as `&mut Person`.
    /// `greeting` only reads through `&self` and cannot call `rename`.
    ///
    /// ```
    /// let mut ada = greet::Person { name: String::from("Ada") };
    /// ada.rename("Grace");
    /// assert_eq!(ada.greeting(), "Hello, Grace!");
    /// ```
    pub fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Take ownership of this person and return the name.
    ///
    /// `self` moves the `Person` into the method. Using the person after
    /// `into_name` does not compile. `greeting` and `rename` only borrow it.
    ///
    /// ```
    /// let ada = greet::Person { name: String::from("Ada") };
    /// assert_eq!(ada.into_name(), "Ada");
    /// ```
    pub fn into_name(self) -> String {
        self.name
    }
}

/// Greet a trimmed name, or explain why the name cannot be greeted.
///
/// `match` has to list every variant. `Ok(person)` names the built person.
/// `Err(message)` names the failure. Deleting the `Err` arm does not compile.
/// `try_new` returns the `Result`. This function handles both cases.
///
/// ```
/// assert_eq!(greet::greet_checked("Ada"), "Hello, Ada!");
/// assert_eq!(greet::greet_checked("   "), "cannot greet: name is empty");
/// ```
pub fn greet_checked(name: &str) -> String {
    match Person::try_new(name) {
        Ok(person) => person.greeting(),
        Err(message) => format!("cannot greet: {message}"),
    }
}

/// Greet a trimmed name, or pass the error back to the caller.
///
/// `?` unwraps `Ok` and continues. On `Err`, it returns that error from this
/// function immediately. The error type has to match this function's `Err`,
/// here `&str`. `greet_checked` matches both arms and always returns a `String`.
///
/// ```
/// assert_eq!(greet::try_greet("Ada").unwrap(), "Hello, Ada!");
/// assert_eq!(greet::try_greet("   "), Err("name is empty"));
/// ```
pub fn try_greet(name: &str) -> Result<String, &'static str> {
    let person = Person::try_new(name)?;
    Ok(person.greeting())
}

/// Greet a trimmed name, or return how long the error message is.
///
/// `try_new` fails with `&str`. This function fails with `usize`. `?` cannot
/// return the `&str` error, because the types differ. `map_err` changes only
/// the `Err` and leaves `Ok` alone. Then `?` can return it.
///
/// ```
/// assert_eq!(greet::try_greet_len("Ada").unwrap(), "Hello, Ada!");
/// assert_eq!(greet::try_greet_len("   "), Err("name is empty".len()));
/// ```
pub fn try_greet_len(name: &str) -> Result<String, usize> {
    let person = Person::try_new(name).map_err(|message| message.len())?;
    Ok(person.greeting())
}

/// Greet a trimmed name in uppercase, or pass the error through.
///
/// `map` changes only the `Ok` value. The closure runs for `Ok(Person)` and
/// turns it into `Ok(String)`. `Err` is left alone, so the closure does not
/// run and the error stays `&str`. `map_err` does the opposite. `try_greet`
/// uses `?` and then wraps a new `Ok`.
///
/// ```
/// assert_eq!(greet::try_greet_upper("Ada").unwrap(), "HELLO, ADA!");
/// assert_eq!(greet::try_greet_upper("   "), Err("name is empty"));
/// ```
pub fn try_greet_upper(name: &str) -> Result<String, &'static str> {
    Person::try_new(name).map(|person| person.greeting().to_uppercase())
}

/// Greet a trimmed name that fits in `limit` characters, or say why not.
///
/// `map` would nest the closure's `Result` inside `Ok`, so the type would be
/// `Result<Result<String, &str>, &str>`. `and_then` flattens that to one
/// `Result`. The closure runs only for `Ok`. An `Err` from `try_new` passes
/// through, and the closure can return its own `Err`.
///
/// ```
/// assert_eq!(greet::try_greet_short("Ada", 3).unwrap(), "Hello, Ada!");
/// assert_eq!(greet::try_greet_short("   ", 3), Err("name is empty"));
/// assert_eq!(greet::try_greet_short("Grace", 3), Err("name is too long"));
/// ```
pub fn try_greet_short(name: &str, limit: usize) -> Result<String, &'static str> {
    Person::try_new(name).and_then(|person| {
        if person.name.len() > limit {
            Err("name is too long")
        } else {
            Ok(person.greeting())
        }
    })
}

/// Greet `name` when it is not blank. Otherwise greet `fallback`.
///
/// `and_then` runs its closure only for `Ok`, and that closure may change the
/// `Ok` type. `or_else` runs its closure only for `Err`. The `Ok` type stays
/// `Person`, so the closure returns `Result<Person, _>`. Returning a greeting
/// from that closure does not compile. `map` afterward turns `Person` into the
/// greeting. An `Ok` from the first `try_new` passes through, so a blank
/// `fallback` is never tried.
///
/// ```
/// assert_eq!(greet::try_greet_or("Ada", "   ").unwrap(), "Hello, Ada!");
/// assert_eq!(greet::try_greet_or("   ", "Grace").unwrap(), "Hello, Grace!");
/// assert_eq!(greet::try_greet_or("   ", "   "), Err("name is empty"));
/// ```
pub fn try_greet_or(name: &str, fallback: &str) -> Result<String, &'static str> {
    Person::try_new(name)
        .or_else(|_| Person::try_new(fallback))
        .map(|person| person.greeting())
}

/// Greet a trimmed name, or wave when the name is blank.
///
/// `unwrap_or` returns the `Ok` string. On `Err` it returns the fallback and
/// drops the error, so `"name is empty"` is gone. The function's type is
/// `String`. `try_greet_or` stays a `Result` and can still fail. The fallback
/// is built even when the name is fine.
///
/// ```
/// assert_eq!(greet::greet_or_wave("Ada"), "Hello, Ada!");
/// assert_eq!(greet::greet_or_wave("   "), "Hello!");
/// ```
pub fn greet_or_wave(name: &str) -> String {
    try_greet(name).unwrap_or(String::from("Hello!"))
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

    #[test]
    fn greet_first_word_matches_both_cases() {
        assert_eq!(greet_first_word("Ada Lovelace"), "Hello, Ada!");
        assert_eq!(greet_first_word("   "), "Hello!");
    }

    #[test]
    fn greet_if_some_handles_one_variant() {
        assert_eq!(greet_if_some("Ada Lovelace"), "Hello, Ada!");
        assert_eq!(greet_if_some("   "), "   ");
    }

    #[test]
    fn describe_matches_each_greeting_variant() {
        let greeting = Greeting::Hello(String::from("Ada"));
        assert_eq!(describe(&greeting), "Hello, Ada!");
        assert_eq!(greeting, Greeting::Hello(String::from("Ada")));
        assert_eq!(describe(&Greeting::Wave), "wave");
    }

    #[test]
    fn person_greeting_borrows_self() {
        let ada = Person {
            name: String::from("Ada"),
        };
        assert_eq!(ada.greeting(), "Hello, Ada!");
        assert_eq!(ada.name, "Ada");
    }

    #[test]
    fn rename_mutably_borrows_self() {
        let mut ada = Person {
            name: String::from("Ada"),
        };
        ada.rename("Grace");
        assert_eq!(ada.name, "Grace");
        assert_eq!(ada.greeting(), "Hello, Grace!");
    }

    #[test]
    fn into_name_takes_ownership_of_self() {
        let ada = Person {
            name: String::from("Ada"),
        };
        let name = ada.into_name();
        assert_eq!(name, "Ada");
    }

    #[test]
    fn new_builds_a_person_without_self() {
        let ada = Person::new("Ada");
        assert_eq!(ada.greeting(), "Hello, Ada!");
        assert_eq!(ada.name, "Ada");
    }

    #[test]
    fn try_new_is_ok_or_err() {
        let ada = Person::try_new(" Ada ").unwrap();
        assert_eq!(ada.name, "Ada");
        assert_eq!(Person::try_new("   "), Err("name is empty"));
    }

    #[test]
    fn greet_checked_matches_ok_and_err() {
        assert_eq!(greet_checked(" Ada "), "Hello, Ada!");
        assert_eq!(greet_checked("   "), "cannot greet: name is empty");
    }

    #[test]
    fn try_greet_returns_the_error_early() {
        assert_eq!(try_greet(" Ada ").unwrap(), "Hello, Ada!");
        assert_eq!(try_greet("   "), Err("name is empty"));
    }

    #[test]
    fn try_greet_len_maps_the_error_before_question_mark() {
        assert_eq!(try_greet_len(" Ada ").unwrap(), "Hello, Ada!");
        assert_eq!(try_greet_len("   "), Err("name is empty".len()));
    }

    #[test]
    fn try_greet_upper_maps_the_ok_value() {
        assert_eq!(try_greet_upper(" Ada ").unwrap(), "HELLO, ADA!");
        assert_eq!(try_greet_upper("   "), Err("name is empty"));
    }

    #[test]
    fn try_greet_short_chains_another_result() {
        assert_eq!(try_greet_short(" Ada ", 3).unwrap(), "Hello, Ada!");
        assert_eq!(try_greet_short("   ", 3), Err("name is empty"));
        assert_eq!(try_greet_short("Grace", 3), Err("name is too long"));
    }

    #[test]
    fn try_greet_or_runs_the_closure_only_on_err() {
        assert_eq!(try_greet_or(" Ada ", "   ").unwrap(), "Hello, Ada!");
        assert_eq!(try_greet_or("   ", " Grace ").unwrap(), "Hello, Grace!");
        assert_eq!(try_greet_or("   ", "   "), Err("name is empty"));
    }

    #[test]
    fn greet_or_wave_unwraps_or_uses_the_fallback() {
        assert_eq!(greet_or_wave(" Ada "), "Hello, Ada!");
        assert_eq!(greet_or_wave("   "), "Hello!");
    }
}

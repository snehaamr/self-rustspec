# Borrow a name, return a greeting

One idea in this crate: a function can read text it does not own, then return a new `String` that it does own.

```rust
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
```

`name: &str` is a borrow. `greet` can look at the characters, and it cannot keep them after it returns. `format!` allocates a new `String`. That new value is what the caller receives.

The test in `src/lib.rs` checks both parts. `name` is still `"Ada"` after `greet(&name)`.

From this directory:

```sh
cargo test
cargo run
```

`cargo test` runs the check. `cargo run` prints `Hello, Rust!`.

Change `Hello` to `Hi` in `greet`, update the test so it expects the new text, and run `cargo test` again. When that change feels obvious, this concept is done.

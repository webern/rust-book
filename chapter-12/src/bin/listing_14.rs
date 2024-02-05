/// Listing 14 breaks the `run` function and `Config` struct out into a `lib.rs` file. We aren't
/// going to bother with that because it would interrupt our ability to iterate in the same Cargo
/// package (because a package can only have one lib.rs file and cause us to bounce around between
/// files.
///
/// Hot Take: It's not really necessary to break your logic out into a `lib.rs` file unless you
/// actually want to publish a library. It *does* allow you to test functions in the separate
/// `tests` directory, but there are ways around that. Namely, you can have a `tests` directory
/// under `src` and conditionally compile that code for `cfg(test)`. This turns out to be simpler
/// than using the canonical integration `tests` directory.
fn main() {}

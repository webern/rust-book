
# Chapter 7

Crate vs. Package

**Package**
- A Rust package has one `Cargo.toml` file.
- A Rust package can define one library (`lib.rs`) and multiple binaries (such as `main.rs`).
- A Rust package can have multiple crates.

**Crate**
- A Rust crate is either a library or a binary.
- It represents an end-product that the compiler will produce (i.e. a lib or bin).

Thank a look at the modules cheat sheet:
https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet

To do the Rustlings exercises:

```shell
git clone git@github.com:webern/rust-book.git
cd rust-book
cd chapter-7/rustlings
cargo run --package rustlings-chapter-7 -- watch
```

Do the exercises in **`10_modules`**.

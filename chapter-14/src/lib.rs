//!
//!
//! # Chapter 14: More About Cargo and Crates.io
//!
//! <https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html>
//!
//! There are no Rustlings exercises for Chapter 14:
//!
//! # Library or Module Level Documentation
//!
//! When you use `//!`, the documentation belongs to the containing scope. So this documentation
//! belongs to the `lib.rs` file and will serve as the introductory documentation to the library.
//!
//! Note that we often use `/*!` for this type of documentation. That is, you can do the same thing
//! with a multiline comment.

#![allow(dead_code, unused_variables, unused_mut)]

fn main() {
    println!("Chapter 14!");
    profiles();
}

/// # Profiles
/// See the workspace `Cargo.toml`
fn profiles() {}

/// This type of comment is a doc comment and will be compiled into HTML documentation. When
/// publishing on crates.io, this documentation will be added to <https://docs.rs>.
///
/// # Examples
///
/// It is common to include an examples section with a code block. This block will turn into a doc
/// test and be executed when you run tests.
///
/// ```
/// use chapter_14::documentation;
///
/// documentation();
/// ```
///
/// # Errors
///
/// The errors section is also common. This function has no errors.
///
/// # Panics
///
/// The panics section is also common. This function does not panic.
///
/// # Compiling Documentation
///
/// You can compile documentation by running `cargo doc --open`.
///
/// # Team Documentation Examples
///
/// - <https://docs.rs/tough/latest/tough/>
/// - <https://docs.rs/crate/coldsnap/latest>
///
pub fn documentation() {}

/// Go through the rest of the sections by reading along in the book:
/// - <https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html>
/// - <https://doc.rust-lang.org/book/ch14-04-installing-binaries.html>
/// - <https://doc.rust-lang.org/book/ch14-05-extending-cargo.html>
pub fn other_sections() {}

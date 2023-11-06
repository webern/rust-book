#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

fn main() {}

use std::fmt::Display;

/// :(
fn longest_with_an_announcement<'a, T>(s: T) -> &'a str
where
    T: AsRef<str> + 'a,
{
    // s.as_ref()
    todo!()
}

#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

fn main() {}

use std::fmt::Display;

/// Lifetimes allow the Rust compiler to prevent dangling references at compile time. This is one
/// of the major inventions of the Rust programming language.
///
/// Lifetime syntax is used when the lifetime of a reference cannot be inferred.
mod lifetimes_intro {}

#[cfg(does_not_compile)]
/// The following does not compile because x does not live long enough.
///
/// This is called a Dangling Reference, and it is what lifetime notation intends to prevent.
fn dangle() {
    let r;

    {
        let x = 5;
        r = &x;
        //  ^^ borrowed value does not live long enough
    }

    println!("r: {r}");
}

#[cfg(does_not_compile)]
/// The borrow checker assigns implicit lifetimes.
/// Lifetimes typically have names like 'a and 'b.
/// 
/// The following does not compile because the lifetime of block 'b is smaller and ends earlier
/// then the outer block 'a.
/// 
#[rustfmt::skip]
fn dangle_2() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
} //                      // ---------+

/// This one compiles because the scope of the reference ends with the println statement.
///
#[rustfmt::skip]
fn dangle_3() {
    let x = 5;            // ----------+-- 'b
                                //           |
    let r = &x;          // --+-- 'a  |
                                //   |       |
    println!("r: {}", r);       //   |       |
                                // --+       |
} //                            // ----------+

/// Lifetime Elision
/// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
/// See the book for historical reasons for elision and rules that are used.
///
/// Lifetime annotations as generics in functions.
///
/// Oftentimes a function does not need lifetime annotation because it is obvious that there is only
/// one lifetime. For example, lifetime annotation is not needed here:
fn no_lifetime_annotation_needed(s: &str) -> &str {
    &s[1..]
}

/// The above function is the same as this, but this annotation is unnecessary.
fn unnecessary_lifetime_annotation<'a>(s: &'a str) -> &'a str {
    &s[1..]
}

/// We use elided lifetimes all the time with `self`
struct Simple(String);

impl Simple {
    /// The lifetime of `self` is always automatically elided when there are no other references.
    fn inner(&self) -> &str {
        &self.0
    }
}

#[cfg(does_not_compile)]
/// Sometimes it is not obvious what which lifetimes are which and annotation is required. This
/// does not compile:
fn missing_lifetimes_does_not_compile(s1: &str, s2: &str, s3: &str) -> (&str, &str) {
    (
        if s1.len() > s2.len() {
            &s1[1..]
        } else {
            &s2[1..]
        },
        &s3[1..],
    )
}

/// Fixing the above function requires lifetime specifiers.
fn required_lifetimes<'a, 'b>(s1: &'a str, s2: &'a str, s3: &'b str) -> (&'a str, &'b str) {
    (
        if s1.len() > s2.len() {
            &s1[1..]
        } else {
            &s2[1..]
        },
        &s3[1..],
    )
}

/// Here is a simpler function from the book that has only one lifetime but requires annotation.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

/// We can use it like this without a problem because both of our strings live long enough.
fn use_longest_1() {
    let s1 = String::from("The longer string");
    {
        let s2 = String::from("short");
        let result = longest(&s1, &s2);

        // This is Ok because we use `result` while both s1 and s2 are still alive.
        println!("{result}");
    }
}

#[cfg(does_not_compile)]
/// This does not compile because of lifetimes.
fn use_longest_2() {
    let s1 = String::from("The longer string");
    let result;
    {
        let s2 = String::from("short");
        result = longest(&s1, &s2);
    }

    // This is not Ok because `result` might reference s2 which does not live long enough.
    println!("{result}");
}

#[cfg(does_not_compile)]
/// Structs always require lifetime annotation if they will hold a reference. This does not compile
/// because it is missing a lifetime annotation.
struct NoLifetime {
    something: &str,
}

/// This struct is Ok because it has a lifetime annotation.
struct WithLifetime<'a> {
    something: &'a str,
}

/// The lifetime is needed in method definitions.
impl<'a> WithLifetime<'a> {
    fn inner(&self) -> &str {
        self.something
    }
}

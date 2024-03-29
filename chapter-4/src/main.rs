#![allow(dead_code, unused_variables, unused_mut)]

fn main() {
    println!("Chapter 4!");
    slice_types::show_me_slices();
}

/// [p 59](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html):
/// - What is a garbage collector?
/// - Does Rust have a garbage collector?
/// - Is this good or bad?
///
/// [p 60](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap)
/// - What is the stack?
/// - What is the heap?
/// - What must we know about data in order to store it on the stack?
/// - What is the advantage of having data on the stack instead of the heap?
///
/// [p 61](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules)
/// - What problem does Rust's ownership model try to solve?
/// (Keeping track of what parts of code are using what data on the heap, minimizing the amount of
/// duplicate data on the heap, cleaning up unused data on the heap.

/// Rust Ownership Rules: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules
/// Each value in Rust has **an owner**.
/// There can only be **one owner at a time**.
/// When the owner goes out of scope, **the value will be dropped**.
mod what_is_ownership {
    /// [p. 62](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-string-type):
    /// - What are the differences between a string literal and a `String` type in Rust?
    /// - When is the a `String`'a memory automatically freed?
    mod the_string_type {}

    /// [p. 64](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#memory-and-allocation):
    /// - What is the name of the special function that Rust calls automatically when a variable
    ///   goes out of scope?
    /// - What is this function called in C++?
    /// - What is this programming principle called (i.e. what is RAII)?
    ///
    /// [p. 65](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move)
    /// What is happening on the stack and on the heap in this code? Is `s1` usable after the
    /// assignment to s2?
    /// ```
    /// let s1 = String::from("hello");
    /// let s2 = s1;
    /// ```
    ///
    /// If Rust were stupid, what would happen when `s1` and `s2` go out of scope?
    /// Instead what happens? (p. 66)
    ///
    /// What choice has Rust made about "deep" copies? (Bottom of p. 66)
    /// Enter `Clone` and `fn clone() -> Self`
    ///
    /// What happens in this code? Is x1 usable after the assignment to x2? Why?
    /// ```
    /// let x1 = 42;
    /// let x2 = x1;
    /// ```
    ///
    /// [p. 67](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-clone):
    /// How is it that we don't need to call clone to get copies of integers and other primitives?
    /// What is special about these primitives that make them so quick to copy?
    /// Enter the `Copy` trait.
    fn memory_and_allocation() {
        // TODO - demo Clone and Copy if there's time
    }

    // Take a look at these two programs and talk through them:
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope

    // Take a look at this program and talk through it:
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions
}

/// [p. 70](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
/// From page 69, we have a long description of a function that takes its parameter by "move"
/// then it has to return that same value by move. Here on page 70 the solution is shown, taking a
/// parameter by reference.
mod references_and_borrowing {
    ////////////////////////////////////////////////////////////////////////////////////////////////

    fn program_1() {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    /// Note `&str` is better then `&String` because it can accept string literals as well.
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    /// The process of creating a reference in Rust is called **BORROWING**.
    ///
    /// [p. 71](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html):
    /// The references are immutable by default.
    fn program_2() {
        let s = String::from("hello");
        change(&s);
    }

    /// Takes an immutable reference. Cannot mutate the string.
    fn change(some_string: &String) {
        // some_string.push_str(", world"); // PROBLEM: does not compile
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    /// # Mutable References
    /// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references
    fn program_3() {
        let mut s = String::from("hello");

        change2(&mut s);
    }

    fn change2(some_string: &mut String) {
        some_string.push_str(", world");
    }

    /// # Mutable Reference Examples
    /// [p. 73](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)
    ///
    /// **Rust can prevent data races at compile time!!!**
    ///
    /// - What is a data race?
    /// - How does Rust prevent data races?
    /// - How many immutable references can you have to one piece of data in one scope?
    ///
    /// > Mutable references have one big restriction: if you have a mutable reference to a value,
    /// > you can have no other references to that value. This code that attempts to create two
    /// > mutable references to s will fail:
    ///
    /// First look at this and talk through it:
    ///
    /// ```no_compile
    /// let mut s = String::from("hello");
    ///
    /// let r1 = &mut s;
    /// let r2 = &mut s;
    ///
    /// println!("{}, {}", r1, r2);
    /// ```
    ///
    /// > Note that a reference’s scope starts from where it is introduced and continues through the
    /// > last time that reference is used. For instance, this code will compile because the last
    /// > usage of the immutable references, the println!, occurs before the mutable reference is
    /// > introduced:
    fn ok_mut_ref_goes_out_of_scope() {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
        } // r1 goes out of scope here, so we can make a new reference with no problems.

        let r2 = &mut s;

        // s.push_str("hi");
        // r2.push_str("hi"); // PROBLEM: Can't have both of these.
    }

    /// [p. 73](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)
    /// Is it Ok to have multiple immutable references?
    fn not_ok_to_have_mut_an_immutable() {
        fn main() {
            let mut s = String::from("hello");

            let r1 = &s; // no problem
            let r2 = &s; // no problem

            // let r3 = &mut s; // BIG PROBLEM
            // println!("{}, {}, and {}", r1, r2, r3);
        }
    }

    /// # Dangling References
    /// [p. 74](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references)
    ///
    /// Explain what is wrong with this code!
    fn let_me_dangle() {
        // let reference_to_nothing = dangle();
    }

    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //
    //     &s
    // }

    /// Preview of future topic, lifetimes: Why is this Ok?
    fn this_does_not_dangle(s: &String) -> &String {
        s
    }

    /// The above function is shorthand for this:
    fn this_has_lifetimes_annotated<'a>(s: &'a String) -> &'a String {
        s
    }

    fn use_the_above_function() {
        let mut my_precious_string = String::from("hello");
        let a_ref_to_the_same = this_does_not_dangle(&my_precious_string);

        // Why can't I mutate my_precious_string?
        // my_precious_string.push_str(" world"); // PROBLEM: Does not compile
        // println!("{}", a_ref_to_the_same);
    }
}

/// # Slice Types
/// [p. 75](https://doc.rust-lang.org/book/ch04-03-slices.html)
mod slice_types {
    /// Let's go through this syntax from p. 76
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    /// Why are slices useful?
    pub(crate) fn show_me_slices() {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..];
        println!("{}", hello);
        println!("{}", world);

        // beware of unicode! This Panics (yikes)!
        // let s = String::from("hell🍺 world");
        //
        // let hello = &s[0..5];
        // let world = &s[6..];
        // println!("{}", hello);
        // println!("{}", world);
    }

    /// Why is &str better than &String in function signatures.
    const CONSTANT: &str = "hello world";

    fn bad_function_signature(s: &String) {
        println!("the string length is: {}", s.len());
    }

    fn good_function_signature(s: &str) {
        println!("the string length is: {}", s.len());
    }

    fn show_why_the_first_function_is_bad() {
        let s = &CONSTANT[6..];

        // easy!
        good_function_signature(s);

        // sad!
        // bad_function_signature(s); // PROBLEM: I can't use it unless I have a String
    }
}

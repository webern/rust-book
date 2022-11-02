fn main() {
    println!("Hello, world!");
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
/// - What problem does Rust's ownership concept try to solve?
/// (Keeping track of what parts of code are using what data on the heap, minimizing the amout of
/// duplicate data on the heap, cleaning up unused data on the heap.
///
mod what_is_ownership {
    /// [p. 62](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-string-type):
    /// - Where is the data stored for Rust's `String` type?
    /// - Where is a string literal stored?
    /// - When is the memory allocated by `String` automatically freed and given back to the
    ///   operating system?
    mod the_string_type {}

    /// [p. 64](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#memory-and-allocation):
    /// - What is the name of the special function that Rust calls automatically when a variable
    ///   goes out of scope?
    /// - What is this called in C++?
    ///
    /// [p. 65](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move)
    /// What is happening on the stack and on the heap in this code?
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
    /// [p. 67](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-clone):
    /// How is it that we don't need to call clone to get copies of integers and other primitives?
    /// What is special about these primitives that make them so quick to copy?
    /// Enter the `Copy` trait.
    fn memory_and_allocation() {
        // TODO - demo Clone and Copy if there's time
    }
}

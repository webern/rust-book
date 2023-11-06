#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

fn main() {}

/// Section 3.1:
/// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability
fn immutable() {
    let x = 1;
    println!("{}", x);

    // Does not compile, x is immutable.
    // x = 2;
}

/// Section 3.1:
/// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability
fn mutable() {
    let mut x = 1;
    println!("{}", x);

    // Ok, x is mutable.
    x = 2;
    println!("{}", x);
}

/// Constants are always immutable.
/// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#constants
///
/// Note: naming convention is SCREAMING_SNAKE_CASE.
///
/// Benefits of constants:
/// - Readability: Seeing the purpose of a value in the code instead of just the value.
/// - Single definition: If you need to change the value, it happens in only one place.
///
const MESSAGE: &str = "Hello World";
const MESSAGE_COUNT: u64 = 2;

fn constants() {
    for _ in 0..MESSAGE_COUNT {
        println!("{}", MESSAGE);
    }
}

/// Constants can only be of types where the size is known at compile-time. Or another way to say it
/// is constants cannot use the heap.
/// This won't compile because a `Vec` uses the heap for variable amount of memory:
// const NO_STRING_CONST: String = String::from("Hello World");

/// Interestingly, an empty `String` can be const because it's size is knowable and the author
/// marked the `new` function as `const`.
const EMPTY_STRING: String = String::new();

/// Shadowing: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
///
/// Surprise! Rust allows you to blow away the existance of a variable and replace it with one that
/// has the same name.
fn shadowing() {
    let x = 1;

    // Does not compile.
    // x = x + 1;

    // No problem! The previous `x` has been dropped because the `let` keyword shadowed it.
    let x = x + 1;

    let mut y = 3;

    // Does not compile: We cannot assign a different type:
    // y = String::from("Hello World");

    // No problem! We can the type because the `let` keyword blows away the previous existence of
    // the variable, just as if it were out of scope.
    let y = String::from("Hello World");
}

/// Section 3.2
/// https://doc.rust-lang.org/book/ch03-02-data-types.html#data-types
///
/// Sometimes we need to annotate the type because it cannot be inferred.
fn annotate_type() {
    // let does_not_compile = "42".parse().expect("Not a number!");
    let does_compile: u32 = "42".parse().expect("Not a number!");
}

/// Various data types, look at them in the book:
/// https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types
fn data_types() {
    // See:
    // https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types
}

/// Section 3.3: Functions
/// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions
///
/// Typed function parameters
fn typed_function_parameters(loop_count: usize, message: &'static str) {
    for i in 0..loop_count {
        println!("{}. {}", i, message);
    }
}

fn calling() {
    // Note that the 10 can be annotated as `usize` but does not need to be in this context.
    typed_function_parameters(10, "Hello World!");
}

/// Statements and Expressions
///
/// Note, this function returns an `i32`
fn statements_and_expressions(decide: i32) -> i32 {
    // This entire let statement and block is an expression. It does not return a value.
    // The scope from `{` to `}` is an expression.
    let mut x = {
        // This is a statement, it does not return a value.
        if decide == 0 {
            // This returns early from the function. The `return` keyword is required here to
            // perform an early return.
            return 42;
        }
        // Statement
        let mut m = decide;

        // This `if` block is a statement.
        if decide > 10 {
            // Statement
            m = m + 1;
        }

        // This line returns the value `m` from this scope, where it is assigned to `x`.
        // This is an expression.
        m
    };

    // This is an expression that returns the value `x` from the function. The `return` keyword is
    // allowed here, but style-wise it should not be used. Note that placing a semicolon after the
    // `x` would turn it into an expression, and it would no longer compile.
    x
}

/// Section 3.4 discusses comments.
/// https://doc.rust-lang.org/book/ch03-04-comments.html#comments
///
/// This current block is a `doc` comment, it uses three slashes.
const THIS_HAS_A_DOC_COMMENT: () = ();

// This is not a doc comment (double slash) and the cargo tooling will not put this comment into
// the documentation for this function.
const NO_DOCUMENTATION: () = ();

/// Section 3.5
/// https://doc.rust-lang.org/book/ch03-05-control-flow.html#using-if-in-a-let-statement
///
/// Then extra note here: if-let
/// https://doc.rust-lang.org/book/ch03-04-comments.html#comments
fn foo() {}

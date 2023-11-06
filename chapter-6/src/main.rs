/*!

# Chapter 6: Enums

pp. 97-110 in the 2018 edition paperback.

Rust enums are inspired by OCaml and Haskell!
They are *algebraic data types*.

 */
#![allow(dead_code, unused_variables, unused_mut)]

fn main() {
    println!("Chapter 6!");
}

/// Enums with and Without Data
mod enums_with_and_without_data {
    /// Plain old Enum
    enum PlainEnumNoData {
        Red,
        Green,
        Blue,
    }

    /// A simple Struct
    struct Point {
        x: u64,
        y: u64,
    }

    /// An enum that can carry various types of data
    enum DataEnum {
        /// This variant does not have any data
        Empty,

        /// This variant carries a named struct. Notice that it is tempting to use the same name
        /// for the data struct and the variant. This is fine but it can get annoying if auto import
        /// functionality gets confused by it.
        Point(Point),

        /// This variant has an unnamed struct. Personally I prefer to use a named struct.
        UnnamedStruct { x: u64, y: u64 },
    }

    // You can implement functions on enums!
    impl DataEnum {
        pub fn explode(&self) {
            match self {
                DataEnum::Empty => todo!(),
                DataEnum::Point(Point { x: 0, .. }) => todo!(),
                DataEnum::UnnamedStruct { x: 0, .. } => todo!(),
                _ => todo!(),
            }
        }
    }

    fn example() {
        let value_1 = DataEnum::Empty;
        let value_2 = DataEnum::Point(Point { x: 0, y: 1 });
        let value_3 = DataEnum::UnnamedStruct { x: 0, y: 1 };

        value_3.explode();
    }
}

/// The Option Enum, p. 101. Does away with `null`.
fn option_unwraps() {
    let null: Option<u8> = None;
    let not_null: Option<u8> = Some(0u8);

    // getting the value from an option.
    // This is panic!
    let x = null.unwrap();

    // This will give the default of the internal type if the option is `None` (in this case 0)
    let x = null.unwrap_or_default();

    // This will give the value of our choice if the option is `None`
    let x = null.unwrap_or(255);

    // This will call the provided lambda function if the option is `None`.
    let x = null.unwrap_or_else(|| {
        // do some expensive calculation to come up with the default
        42
    });
}

enum Letter {
    A,
    B,
    C,
}

/// Exhaustive Matches p. 104
///
/// Note: The `match` operator is like `case` in other languages.
///
fn exhaustive_matches() {
    // We must cover all possible code branches when using the match operator

    // Won't compile!
    // let letter = Letter::C;
    // match letter {
    //     Letters::A => println!("A"),
    //     Letters::B => println!("B"),
    //     // oh, no, we haven't covered all cases
    // }

    // This is Ok because we cover all cases:
    let letter = Letter::B;
    match letter {
        Letter::A => println!("A"),
        Letter::B => println!("B"),
        Letter::C => println!("C"),
    }

    // This is also Ok because we use the "catch all" case:
    match letter {
        Letter::A => println!("A"),
        _ => println!("something else"),
    }

    // Covering all conditions applies to any type. We can match on a &str:
    let some_str = "hello";

    // This won't compile!
    // match some_str {
    //     "hello" => println!("hello world"),
    // }

    // This uses the "catch all" so it's fine:
    match some_str {
        "hello" => println!("-------------- HELLO"),
        _ => println!("unknown string"),
    }

    // Here's some very interesting syntax. You can introduce a variable in the match arms:
    match some_str {
        "hello" => println!("-------------- HELLO"),
        new_str_variable => println!("{}", new_str_variable), // prints the value of some_str
    }

    // Here's a subtle mistake that can be very confusing. It's a bug because I forgot to use
    // `Letters::` in front of the `A`. I'm writing `A` instead of `Letter::A` and the compiler
    // thinks it is a variable named `A`.
    //
    // This will **ALWAYS** print "A"/
    // Fortunately the compiler warns me that the _ => case is unreachable.
    // match letter {
    //     A => println!("A"),
    //     _ => println!("something else"),
    // }
}

enum Value {
    I(u64),
    S(&'static str),
}

/// Patterns that bind to values. p. 105
///
/// Pattern matching (i.e. using the `match` operator) can get pretty deep syntax-wise. You can
/// match a certain variant only for certain values, for example:
fn patterns_that_bind_to_values() {
    let value = Value::S("hello");
    match value {
        Value::S("hello") => println!("I know the value was 'hello'"),
        Value::I(42) => println!("I know the value was 42)"),
        _ => println!("I need the catch-all because I haven't covered all values"),
    }

    // We can also assign whatever is "in" the variant to a variable:
    match value {
        Value::I(i) => println!("{}", i),
        Value::S(s) => println!("{}", s),
    }
}

// If let. These two blocks of code are equivalent
fn if_let() {
    let value = Some(42);
    {
        match value {
            Some(i) => println!("the value is {}", i),
            _ => { /* do nothing */ }
        }
    }

    {
        if let Some(i) = value {
            println!("the value is {}", i);
        }
    }
}

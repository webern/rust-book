/*!

# Chapter 18: Pattern Matching

*/
#![allow(dead_code, unused_variables, unused_mut)]

/// # Patterns and Matching
///
/// ## Patterns
///
/// Patterns are a special syntax in Rust for matching against the structure of types, both complex
/// and simple. Using patterns in conjunction with match expressions and other constructs gives you
/// more control over a program’s control flow. A pattern consists of some combination of the
/// following:
///
/// - Literals
/// - Destructured arrays, enums, structs, or tuples
/// - Variables
/// - Wildcards
/// - Placeholders
///
/// Some example patterns include `x`, `(a, 3)`, and `Some(Color::Red)`.
///
fn main() {
    println!("Chapter 18: Pattern Matching");
    sep();
    if_let();
    match_arms();
    sep();
    while_let();
    destructuring();
    sep();
    matching_named_variables();
    sep();
    ranges();
}

/// ## Destructuring Structs
fn destructuring_structs() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // Creates variables a and b that are assigned values of p.x and p.y.
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Captures p.x and p.y in the new variables x and y.
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // Matching on the values of x and y.
    match p {
        Point { x: value, y: 0 } => println!("On the x axis at {value}"),
        Point { x: 0, y: value } => println!("On the y axis at {value}"),
        Point { x: x, y: y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Enums can also be destructured!
    enum SimpleMessage {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = SimpleMessage::ChangeColor(0, 160, 255);

    match msg {
        SimpleMessage::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        SimpleMessage::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        SimpleMessage::Write(text) => {
            println!("Text message: {text}");
        }
        // Look here! We have destructured the tuple inside the enum variant.
        SimpleMessage::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    // Nested, oh my!

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    // You can ignore a values
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    // Ignore certain values, another example
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // Ignoring the rest of a struct with ..
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3d { x: 0, y: 0, z: 0 };

    match origin {
        Point3d { x, .. } => println!("x is {}", x),
    }

    // Using a match guard (if) in a match arm
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // Another if example
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    // Another if example
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @Bindings Oh My!
}

/// ## @ Bindings
///
/// The at operator @ lets us create a variable that holds a value at the same time as we’re testing
/// that value for a pattern match. In Listing 18-29, we want to test that a Message::Hello id field
/// is within the range `3..=7`. We also want to bind the value to the variable `id_variable` so we
/// can use it in the code associated with the arm. We could name this variable id, the same as the
/// field, but for this example we’ll use a different name.
///
fn at_bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

/// ## Match Arms
///
/// Patterns can be used in match arms, which are like case statements in other languages.
///
/// The syntax is:
///
/// ```text
/// match VALUE {
///     PATTERN => EXPRESSION,
///     PATTERN => EXPRESSION,
///     PATTERN => EXPRESSION,
/// }
/// ```
///
fn match_arms() {
    let x = Some(1);

    // The match arms are evaluated in order:
    let plus_one = match x {
        None => None,
        // This arm will match only if the inside value equals 42.
        Some(42) => panic!("The answer has been found"),
        // This arm will match with any value inside the Option, and captures that value in the
        // variable i.
        Some(i) => Some(i + 1),
    };
}

/// ## If Let
///
/// This construct horks the brain a little bit, but it is a combination of pattern matching with
/// an if statement and, typically, capturing the value inside an enum variant.
///
fn if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

/// ## While Let
///
/// Similar to `if let`, but this time serving as the conditional in a `while` loop.
///
fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

/// ## Destructuring
///
/// Not sure exactly why this is considered a "pattern", but the chapter shows how you can
/// destructure a tuple (or a struct, or whatever, I suppose), in combination with a `for` loop.
fn destructuring() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

/// ## Refutability
///
/// If a pattern will match with any value, it is called "irrefutable". For example:
fn refutability() {
    // Won't compile because the pattern is refutable, i.e. what would happen if the value was None?
    // let refutable_value: Option<i32> = None;
    // let Some(x) = refutable_value;

    // This works because the if statement allows us to use a refutable pattern:
    let value = Some(1);
    if let Some(x) = value {}
}

/// # Pattern Syntax
///
/// ## Matching Literals
///
/// Note that the last arm is irrefutable. This is required so that matching is exhaustive.
fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/// ## Matching Named Variables
///
/// CAUTION: matches start a new scope and can shadow variables. See how `y` gets shadowed here.
///
fn matching_named_variables() {
    println!("Matching and Variables");
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

/// ## Multiple Patterns (OR)
///
fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/// ## Ranges (..=)
///
fn ranges() {
    let x = 5;

    match x {
        1..=5 => println!("one through five (inclusive)"),
        _ => println!("something else"),
    }

    // Or with chars!
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

/// Used to separate printed things.
fn sep() {
    println!();
    println!("----------------------------------------------------------------------------------");
    println!();
}

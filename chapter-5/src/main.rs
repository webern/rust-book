/*!

# Chapter 5: Structs

pp. 83-96 in the 2018 edition paperback.

 */
#![allow(dead_code, unused_variables, unused_mut)]

use std::fmt::{Display, Formatter};

fn main() {
    println!("Chapter 5!");
    use_the_empty_struct();
}

/// ```
///
/// ```
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

/// The following struct is actually no different than this tuple other than labeling the data.
fn tuples_are_like_structs() {
    let user = (
        String::from("foo@bar.com"),
        String::from("foobar"),
        false,
        0,
    );

    // The above is the same as the following, but the following struct is easier to understand.
    let user = User {
        email: String::from("foo@bar.com"),
        username: String::from("foobar"),
        active: false,
        sign_in_count: 0,
    };
}

/// [p. 85]:(https://doc.rust-lang.org/book/ch05-01-defining-structs.html#defining-and-instantiating-structs)
/// Listing 5-4: This is a strange way to show a constructor pattern...
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

/// Normally you would see this instead... this is best practice:
impl User {
    pub fn new(email: String, username: String) -> Self {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}

/// Shorthand syntax (still on p. 85): Notice that the two following functions are equivalent.
fn no_shorthand(email: String, username: String) -> User {
    // Clippy or your IDE might complain because you don't need to repeat email and username.
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

/// It is more idiomatic to use this shorthand:
fn shorthand(email: String, username: String) -> User {
    // Now Clippy and your IDE should be happy.
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

/// Shorthand struct copying syntax (Struct update syntax)
/// [p. 86](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax)
fn increment_sign_in_count(user: User) -> User {
    // We can copy the fields from one User struct to another with shorthand:
    User {
        sign_in_count: user.sign_in_count + 1,
        ..user
    }
}

struct SuperUser {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
    permission_level: u16,
}

// It would be nice if I could do this, but I can't :(
// There is an [RFC](https://rust-lang.github.io/rfcs/2528-type-changing-struct-update-syntax.html).
// fn super_user_from_user(user: User) -> SuperUser {
//     SuperUser{
//         permission_level: 2,
//         ..user
//     }
// }

// Tuple Structs
// [p. 86](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types)

/// A point consisting of X, Y and Z
struct Point(u64, u64, u64);

/// A color consisting of Red, Green and Blue
struct Color(u64, u64, u64);

fn use_point(point: &Point) {
    println!("X is {}", point.0);
    println!("Y is {}", point.1);
    println!("Z is {}", point.2);
}

fn use_color(color: &Color) {
    println!("Red is {}", color.0);
    println!("Green is {}", color.1);
    println!("Blue is {}", color.2);
}

/// `Point` and `Color` can **NOT** be used interchangeably
fn not_interchangeable() {
    let point = Point(1, 2, 3);
    let color = Color(1, 2, 3);

    // No! doesn't work
    // use_point(&color);
    // use_color(&point);

    // Yes! this works
    use_point(&point);
    use_color(&color);
}

/// Unit-like structs without any fields:
struct Hello;

// Cool!, I can impl traits on this empty struct
impl AsRef<str> for Hello {
    fn as_ref(&self) -> &str {
        "Hello"
    }
}

impl Display for Hello {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt("World!", f)
    }
}

fn use_the_empty_struct() {
    let h = Hello;

    // Prints "Hello World!"
    println!("{} {}", h.as_ref(), h)
}

/// Ownership of struct data, Lifetime **Preview**
/// [p.87](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#ownership-of-struct-data)
struct BorrowedData<'a> {
    email: &'a str,
    username: &'a str,
}

fn use_borrowed_data_struct() {
    // Data is owned by these variables:
    let email = String::from("foo@example.com");
    let username = String::from("foo");

    // Assign references to the struct data. Compiler is happy because the lifetime of the variables
    // is longer/same-as the lifetime of the struct.
    let borrowed_data = BorrowedData {
        email: &email,
        username: &username,
    };

    println!("{} {}", borrowed_data.email, borrowed_data.username)
}

// Display and Debug p. 91
// Instructions: for this example show the compiler error, then implement Debug and Display manually
// then use the derive macro for Debug

struct PrintMe {
    first_word: String,
    second_word: String,
}

// This doesn't compile unless we implement Debug and Display
// fn print_me() {
//     let value = PrintMe {
//         first_word: "Hello".to_string(),
//         second_word: "World".to_string(),
//     };
//     // Uses Display
//     println!("{}", value);
//
//     // Uses Debug
//     println!("{:?}", value);
// }

// Different selfs

pub struct WrappedString {
    value: String,
}

impl WrappedString {
    /// Capitalized `Self` represent the type, i.e. `WrappedString`.
    pub fn new<S: Into<String>>(value: S) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// `&mut self` does a mutable borrow of the instance.
    pub fn mutate(&mut self) {
        self.value.push_str(" foo");
    }

    /// `&self` does an immutable borrow of the instance.
    pub fn print(&self) {
        println!("{}", self.value)
    }

    /// `self` "consumes" the instance. i.e. destroys it by moving it into the function.
    /// i.e. `wrapped_string.be_like_a_builder()` is similar to `be_like_a_builder(wrapped_string)`
    /// taking its input by `move` (i.e. not by reference).
    fn be_like_a_builder(self) -> Self {
        Self {
            value: format!("{} bar", self.value),
        }
    }
}

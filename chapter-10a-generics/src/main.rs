#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

fn main() {
    function_generics::use_largest();
    struct_generics::use_point_b_more();
}

/// # Generics in Functions p. 174
mod function_generics {
    /// Example in book. I've included the traits necessary for it to compile.
    fn largest<T>(list: &[T]) -> T
    where
        T: Copy + PartialOrd,
    {
        // Note this can panic, don't do this in production code.
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    pub(super) fn use_largest() {
        let number_list = [4, 3, 2, 5, 1];
        let string_list = ["foo", "bar", "baz"];
        let largest_number = largest(&number_list);
        let largest_string = largest(&string_list);
        println!("the largest number is {}", largest_number);
        println!("the largets string is {}", largest_string);
    }
}

/// # Generics in Structs p. 174 and Methods p. 179
mod struct_generics {

    /// A struct with the same type for both fields.
    struct PointA<T> {
        x: T,
        y: T,
    }

    fn use_point_a() {
        // doesn't compile because we have used two different types
        // let point_1 = PointA { x: 1, y: 1.0 };

        // does compile because both types are the same.
        // note: we haven't done anything to constrain what T can be, so strings are fine!
        let point_2 = PointA { x: "foo", y: "bar" };
    }

    /// A struct with multiple types.
    struct PointB<T, U> {
        x: T,
        y: U,
    }

    /// Weird but fine.
    fn use_point_b() {
        let point = PointB { x: 1, y: "bar" };
    }

    // If we want to write associated functions in the impl block, they look like this.
    // Note the syntax repeats the type parameter names. At "impl" the existence of the
    // generics is declared, at PointB we are "using" them as needed in the struct.
    impl<T, U> PointB<T, U> {
        pub fn new(x: T, y: U) -> Self {
            Self { x, y }
        }

        /// Returns whatever type Self<T> is.
        pub fn x(&self) -> &T {
            &self.x
        }

        /// Returns whatever type Self<U> is.
        pub fn y(&self) -> &U {
            &self.y
        }
    }

    pub(super) fn use_point_b_more() {
        // The types can be inferred.
        let point = PointB::new("foo", 1.0);
        println!("point_b x is: {}", point.x());
        println!("point_b y is: {}", point.y());
    }
}

/// ## Generics in Impls, Functions for only certain types
mod partial_impls {
    use std::fmt::Display;

    struct Value<T> {
        inner: T,
    }

    // This impl block will exist for any type T. That is, every instantiation of the `Value` struct
    // will have a `get` function.
    impl<T> Value<T> {
        fn get(&self) -> &T {
            &self.inner
        }
    }

    // This impl block will only exist when T is an `i32`. The `is_less_than_zero` function will not
    // exist when `T` is a `String`, or a `u32`, or any type other than `i32`.
    impl Value<i32> {
        fn is_less_than_zero(&self) -> bool {
            self.inner < 0
        }
    }

    // This impl block will only exist when `T` is a type that implements the `Display` trait. For
    // example `String` implements `Display` but `Option<String>` does not.
    impl<T: Display> Value<T> {
        fn print(&self) {
            println!("{}", self.inner)
        }
    }

    fn use_in_examples() {
        let t_is_an_i32 = Value { inner: 1 };
        let t_is_a_str = Value { inner: "Hi" };
        let t_is_an_option = Value { inner: Some("Hi") };

        // T is an i32 so we have the less_than_zero function
        let _ = t_is_an_i32.is_less_than_zero();
        // T is displayable, so we have the print function
        t_is_an_i32.print();

        // T is a str, not an i32, so we do not have the `is_less_than_zero` function.
        // t_is_a_str.is_less_than_zero();

        // Does not compile: T is not displayable so we do not have the print function.
        // t_is_an_option.print();
    }
}

/// # Discussion of Performance p. 181
///
/// - When generics are used, the types being used are "substituted" for the generic params and code
///   is generated. (Called monomorphism)
/// - We therefore pay **NO** runtime cost for generics. Generic code is just as fast as non-generic
///   code. (Unlike Java?)
/// - We **DO** pay a cost in compile time. Each usage of a generic function, struct, enum, etc used
///   with a different type causes the function, struct, enum etc to be compiled an additional time.
mod performance {
    use std::fmt::Display;

    // This function...
    fn print<T: Display>(value: T) {
        println!("I'm printing this: {}", value)
    }

    // When used like this...
    fn use_print() {
        print(1);
        print("str");
    }

    // Causes this code to be generated...
    fn print_i32(value: i32) {
        println!("I'm printing this: {}", value)
    }

    // And this code to be generated...
    fn print_str(value: &str) {
        println!("I'm printing this: {}", value)
    }
}

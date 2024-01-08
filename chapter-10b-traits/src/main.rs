#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

///! # Chapter 10b - Traits
///
/// Traits are often called *interfaces* in other languages (like C++, Go and Java). Traits are
/// different in that they represent constraints on *generic types*.
///
/// With *interfaces* we are talking about pointers to objects that are known to have certain
/// functions available dynamically at runtime by pointing to a certain place in the virtual
/// function table.
///
/// With *generics* and *traits*, we are generating specific code at compile time, with no runtime
/// cost (i.e. no virtual function table lookup), and constraining what types can be used when
/// generating code.
///
/// C++ introduced something like traits in C++20. These are called `concepts`, `requirements` and
/// `constraints`:
/// - https://en.cppreference.com/w/cpp/language/constraints
/// - https://en.cppreference.com/w/cpp/language/requires
///
/// When `Go` introduced generics in `1.18`, they included a similar idea called `constraints`:
/// https://go.dev/blog/intro-generics
///
fn main() {
    defining_a_trait::do_greetings();
    conditionally_implement_methods::use_conditional_impl();
    blanket_impls::call_add_and_print();
}

/// p. 183
mod defining_a_trait {
    use std::borrow::Cow;
    use std::fmt::Display;

    /// Trait functions can include a default implementation.
    pub trait Greet {
        /// This must be implemented when you implement the trait.
        fn name(&self) -> Cow<'_, str>;

        /// This haa a "default" implementation. You can override it, or leave it out (to get the
        /// default) when you impl the trait.
        fn greet(&self) {
            // We can use other trait functions in the default implementation.
            println!("Hello: {}", self.name())
        }
    }

    /// Here we have a struct that stores a persons first and last name and concatenates them
    /// for `Greet` `name()`. The default implementation of `greet()` works because it calls the
    /// `name()` function, which we have implemented.
    struct FullName {
        first_name: String,
        last_name: String,
    }

    impl Greet for FullName {
        fn name(&self) -> Cow<'_, str> {
            Cow::Owned(format!("{} {}", self.first_name, self.last_name))
        }
    }

    /// Here we have a struct that only stores a single `String` as the name and returns that for
    /// `Greet` `name()`.
    struct Nickname {
        value: String,
    }

    impl Greet for Nickname {
        fn name(&self) -> Cow<'_, str> {
            Cow::Borrowed(&self.value)
        }
    }

    /// Here we have a struct that overrides the `greet()` function.
    struct Different {}

    impl Greet for Different {
        fn name(&self) -> Cow<'_, str> {
            Cow::Borrowed("I'm different")
        }

        /// We can override the default implementation.
        fn greet(&self) {
            println!("I'm doing something different")
        }
    }

    pub fn do_greetings() {
        let fullname = FullName {
            first_name: "Leonardo".to_string(),
            last_name: "Dunlap".to_string(),
        };
        let nickname = Nickname {
            value: "Leo".to_string(),
        };
        let different = Different {};

        fullname.greet();
        nickname.greet();
        different.greet();
    }
}

/// p. 184
mod traits_must_be_local {
    mod crate_a {
        pub trait Foo {
            fn foo(&self) -> String;
        }
    }

    mod crate_b {
        pub struct Bar {}
    }

    /// If these were actually crates (and not modules) then this would not compile.
    mod crate_c {
        use crate::traits_must_be_local::crate_a::Foo;
        use crate::traits_must_be_local::crate_b::Bar;

        // This would not compile if the `Foo` trait were in a different Rust package.
        impl Foo for Bar {
            fn foo(&self) -> String {
                todo!()
            }
        }

        // This compiles because Rust allows us to impl traits for some `std` types.
        impl Foo for Vec<i32> {
            fn foo(&self) -> String {
                todo!()
            }
        }

        // This is always fine. We can implement someone else's trait on a type as long as we own
        // the type.
        struct MyBar {
            bar: Bar,
        }

        impl Foo for MyBar {
            fn foo(&self) -> String {
                todo!()
            }
        }
    }
}

/// Default Implementations
///
/// Here's an example of how we used default implementations in testsys:
/// https://github.com/bottlerocket-os/bottlerocket-test-system/blob/develop/model/src/clients/crd_client.rs
mod default_implementations {}

/// Traits as Parameters p. 186
mod traits_as_parameters {
    trait Name {
        fn name(&self) -> &str;
    }

    // These are equivalent. Requiring a type to implement a trait is called a "trait bound".

    fn print_name_a(name: impl Name) {}
    fn print_name_b<N: Name>(name: N) {}
    fn print_name_c<N>(name: N)
    where
        N: Name,
    {
    }
}

/// Return Types p. 188
mod returning_generics_with_trait_bounds {
    use std::fmt::Display;
    use std::path::PathBuf;

    /// This can return a `&str`, or anything else that implements `Display`.
    fn something_displayable() -> impl Display {
        "this is a displayable type"
    }

    /// However, it can only return one type. It is like a generic `T` that cannot represent
    /// multiple types. The following will not compile.
    fn only_one_return_type_allowed() -> impl Display {
        // This code path return a &str, which implements `Display`, but conflicts with the other
        // return type. DOES NOT COMPILE
        // if PathBuf::from("/whatever").is_file() {
        //     return "return a &str";
        // }

        // This code path returns an i32, which implements `Display`
        5
    }
}

/// Trait Bounds to Conditionally Implement Methods p. 191
mod conditionally_implement_methods {
    use std::fmt::{Display, Formatter};

    /// This struct is a thin wrapper around any type (kind of useless but...)
    struct Anything<T>(T);

    // If the type held by `Anything` implements `Display` then we can implement `Display`
    // conditionally for `Anything`.
    impl<T: Display> Display for Anything<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            // Pass through the display responsibility to the inner value.
            Display::fmt(&self.0, f)
        }
    }

    /// This is a struct that does NOT implement `Display`
    struct NotDisplayable;

    /// This is a struct the DOES implement `Display`.
    struct Displayable;

    impl Display for Displayable {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "I am a displayable struct.")
        }
    }

    /// This function shows that some `Anything` specializations can be displayed and others cannot.
    pub(super) fn use_conditional_impl() {
        let displayable_string_inner = Anything("I am a displayable string");
        let displayable_struct = Anything(Displayable);
        let non_displayable = Anything(NotDisplayable);

        println!("use_conditional_impl: {}", displayable_string_inner);
        println!("use_conditional_impl: {}", displayable_struct);

        // DOES NOT COMPILE
        // println!("use_conditional_impl: {}", non_displayable);
    }
}

/// Blanket Impls p. 192
mod blanket_impls {
    use std::fmt::{Display, Formatter};
    use std::ops::Add;

    /// Define a trait that prints the result of adding itself to itself in the form "{} + {} = {}".
    /// For example, if `self` is the number `4`, then `add_and_print()` prints "4 + 4 = 8".
    trait PrintAddSelfToSelf {
        fn add_and_print(&self);
    }

    // Because the trait is defined in our own crate, we can make a blanket implementation for all
    // types that are displayable and addable.
    impl<T: Add<Output = T> + Display + Clone> PrintAddSelfToSelf for T {
        fn add_and_print(&self) {
            let sum = self.clone() + self.clone();
            println!("{} + {} = {}", self, self, sum);
        }
    }

    /// Because of the blanket impl we can call `add_and_print` on lots of things.
    pub fn call_add_and_print() {
        5.add_and_print();
        7u64.add_and_print();

        /// I can create some type that will also work.
        #[derive(Clone)]
        struct MyType(String);

        impl Display for MyType {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        impl Add for MyType {
            type Output = MyType;

            fn add(self, rhs: Self) -> Self::Output {
                MyType(self.0.clone() + self.0.as_str())
            }
        }

        // I can call `add_and_print` on `MyType` because it implements the right traits.
        MyType("abc".to_string()).add_and_print();
    }
}

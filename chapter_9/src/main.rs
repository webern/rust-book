#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use crate::mane::ManeError;

/// # Getting Help from the Compiler (p. 156)
/// You can use this trick when you don't know the type of something. Annotate a `let` statement
/// with some type, even though you know it is wrong, then compile. The compiler will tell you
/// what the actual type is.
mod type_discovery_trick {
    /// Ignore this
    const ARR: [&str; 2] = ["foo", "bar"];

    /// A function that returns a type I'm not sure about
    fn what_type_does_this_return() -> std::slice::Iter<'static, &'static str> {
        ARR.iter()
    }

    /// I can use the compiler to tell me what the exact type is.
    fn function() {
        // What if I need to annotate this type and don't know that type is being returned?
        let iter = what_type_does_this_return();
    }
}

/// # Creating an Error from Scratch
/// We typically use libraries like `Snafu`, but let's make an error from scratch.
mod custom_error {
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    // Technically speaking, anything can be used as an error.
    fn dumb_error_type() -> Result<(), u64> {
        // This is a dumb error type because it is not idiomatic. What do we do with `1`?
        Err(1)
    }

    /// Instead error types should implement the std library `Error` trait. This is still a weird
    /// error, but at least it implements the correct stuff to be a proper Rust error.
    #[derive(Debug)]
    pub struct ErrorMessage(String);

    // Technically nothing is required beyond `Debug`
    impl Display for ErrorMessage {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            Display::fmt(&self.0, f)
        }
    }

    impl Error for ErrorMessage {
        // technically there is nothing else we must do here. below we will see how we should
        // actually implement (override) the `source` function.
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    /// The above error is fine, but it's missing something important. The `Error` trait allows you
    /// to implement a `source` function which returns the underlying error that is being "wrapped".
    /// This allows for a pseudo-stack-trace of errors. Here's how you can implement a more
    /// functional error.
    #[derive(Debug)]
    pub struct BetterError {
        /// The error message that we want to include. This may be adding context to an underlying
        /// error held in `source`.
        message: String,

        /// The underlying error that is being wrapped by `BetterError` if one exists. Note that it
        /// is very common and idiomatic to require error types to also implement `Send` and `Sync`.
        /// Without `Send` and `Sync` users will struggle to use your library in `async` or multy-
        /// threaded applications. Using `Box<dyn>` allows us to hold a pointer to any type of
        /// error that implements these traits.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    }

    // We either write the `message` by itself, or we write it along with the underlying error's
    // message.
    impl Display for BetterError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self.source.as_ref() {
                None => Display::fmt(&self.message, f),
                Some(source) => write!(f, "{}: {}", self.message, source),
            }
        }
    }

    impl Error for BetterError {
        /// The default trait implementation of this function simply returns `None`. Here we are
        /// overriding it so that our error "plays nice" and returns any underlying error. Through
        /// this mechanism, a chain of wrapping errors is built up that can serve as a pseudo-
        /// stack-trace.
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            // This is ugly and took by a long time to figure out, but we are just casting our
            // `source` to the type that is defined by the `Error` trait.
            self.source.as_ref().map(|e| e.as_ref() as &(dyn Error))
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    /// An even better Error would give the user some enum to explain the error condition.
    #[derive(Debug, Clone, Copy)]
    pub enum Kind {
        /// The universe collapse.
        Implosion,
        /// The universe didn't collapse.
        HeatDeath,
    }

    #[derive(Debug)]
    pub struct BestError {
        message: String,
        kind: Kind,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    }

    impl BestError {
        /// We give the user a public interface where they can figure out what happened (this is
        /// like `std::io::Error`)
        pub fn kind(&self) -> Kind {
            self.kind
        }
    }

    impl Display for BestError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self.source.as_ref() {
                None => Display::fmt(&self.message, f),
                Some(source) => write!(f, "{}: {}", self.message, source),
            }
        }
    }

    impl Error for BestError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.source.as_ref().map(|e| e.as_ref() as &(dyn Error))
        }
    }
}

/// # The Question Mark Operator (p.160)
/// What is the `?` operator actually doing?
mod question_mark_operator {
    fn result() -> Result<(), ()> {
        Err(())
    }

    // These are the same
    fn verbose() -> Result<(), ()> {
        let _ = match result() {
            Ok(ok) => ok,
            Err(e) => return Err(e),
        };
        Ok(())
    }

    fn shorthand() -> Result<(), ()> {
        let _ = result()?;
        Ok(())
    }
}

/// # The Question Mark Operator Calls Into (p.162)
/// The `?` operator can convert errors if they implement `From`.
/// For brevity the errors in this example are not "proper errors".
mod question_mark_into_call {
    struct ErrorTypeOne;
    struct ErrorTypeTwo;

    impl From<ErrorTypeOne> for ErrorTypeTwo {
        fn from(e: ErrorTypeOne) -> Self {
            // pretend there is some real transformation here
            Self
        }
    }

    fn returns_error_type_one() -> Result<(), ErrorTypeOne> {
        Err(ErrorTypeOne)
    }

    fn returns_error_type_two() -> Result<(), ErrorTypeTwo> {
        // the conversion to ErrorTypeTwo is automatic when ? is used
        Ok(returns_error_type_one()?)
    }

    struct ErrorTypeThree;

    // This will not compile because ErrorTypeOne cannot be automatically converted to
    // ErrorTypeThree
    // fn returns_error_type_three() -> Result<(), ErrorTypeThree> {
    //     Ok(returns_error_type_one()?)
    // }
}

/// # Errors and the `main` Function (p. 164)
/// The main function will show an error using `Debug`, which is too bad.
/// This is why we usually catch the error from a `run` function and display it with `eprintln!()`.
fn main() -> Result<(), ManeError> {
    mane::run_program()
}

mod mane {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};

    pub fn run_program() -> Result<(), ManeError> {
        println!("Chapter 9!");
        Err(ManeError)
    }

    pub struct ManeError;

    impl Display for ManeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            Display::fmt("error is displayed", f)
        }
    }

    impl Debug for ManeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            Display::fmt("error is debugged", f)
        }
    }

    impl Error for ManeError {}
}

/// # Custom Types for Validation (p. 167)
/// In Rust it is idiomatic to prevent a function from accepting bad input by making it impossible
/// with the type system.
mod type_system_validation {
    /// This is a terrible function. It accepts data that can be bad then validates.
    pub fn bad_guess_between_1_and_10(guess: u8) -> bool {
        if guess < 1 || guess > 10 {
            panic!("the guess was '{}' but should be between 1 and 10", guess);
        }
        guess == 5
    }

    /// This is a better design. We create a type that can only ever be valid.
    pub struct Clamp<const MIN: u8, const MAX: u8>(u8);

    impl<const MIN: u8, const MAX: u8> Clamp<MIN, MAX> {
        fn new(input: u8) -> Self {
            if input < MIN {
                Self(MIN)
            } else if input > MAX {
                Self(MAX)
            } else {
                Self(input)
            }
        }
    }

    /// Now this function can never accept bad input.
    pub fn good_guess_between_1_and_10(guess: Clamp<1, 10>) -> bool {
        guess.0 == 5
    }
}

/// # Demo Snafu - Problematic Usage
/// This is problematic because the enum exposes implementation details in the public interface.
mod snafu_bad {
    use snafu::Snafu;
    use std::path::PathBuf;

    /// Exposes too much information about our implementation details. We need to add a variant
    /// for any different message or data that we might want.
    #[derive(Debug, Snafu)]
    pub enum Error {
        #[snafu(display("Unable to canonicalize path '{}': {}", path.display(), source))]
        UnableToOpen {
            path: PathBuf,
            source: std::io::Error,
        },

        #[snafu(display("Failed to so something: {}", source))]
        Something { source: std::io::Error },
    }
}

/// # Demo Snafu - Better Usage
/// Snafu has a better mode where the snafu enum variants are hidden.
mod snafu_good {
    use snafu::Snafu;
    use std::path::PathBuf;

    /// We have a public wrapper that hides our snafu enum.
    #[derive(Debug, Snafu)]
    pub struct Error(PrivateError);

    /// We can do whatever we want with the public interface of our error. For example we can create
    /// a stable, purpose-build enum for our users.
    pub enum Kind {
        Io,
        Implosion,
        HeatDeath,
    }

    impl Error {
        pub fn kind(&self) -> Kind {
            match &self.0 {
                PrivateError::UnableToOpen { .. } | PrivateError::Something { .. } => Kind::Io,
            }
        }
    }

    /// This is kept private.
    #[derive(Debug, Snafu)]
    enum PrivateError {
        #[snafu(display("Unable to canonicalize path '{}': {}", path.display(), source))]
        UnableToOpen {
            path: PathBuf,
            source: std::io::Error,
        },

        #[snafu(display("Failed to so something: {}", source))]
        Something { source: std::io::Error },
    }
}

// TODO - # Demo Anyhow

// TODO - # Demo thiserror

// TODO - # Demo Eyre

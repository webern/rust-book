#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use std::panic::catch_unwind;
pub mod error_libraries;
pub mod helpers;
pub mod mane_error;

/// ## Panic vs Error
///
/// - Panic: will print a failure message, unwind, clean up the stack, and quit the program.
/// - Error: a type (trait actually) used by convention to report a problem to the calling function.
///
/// You can actually catch a panic, but shouldn't. Here's what the [documentation] says:
/// [documentation]: https://doc.rust-lang.org/std/panic/fn.catch_unwind.html
///
/// > It is currently undefined behavior to unwind from Rust code into foreign code, so this
/// > function is particularly useful when Rust is called from another language (normally C). This
/// > can run arbitrary Rust code, capturing a panic and allowing a graceful handling of the error.
/// >
/// > > It is not recommended to use this function for a general try/catch mechanism. The Result
/// > type is more appropriate to use for functions that can fail on a regular basis. Additionally,
/// > this function is not guaranteed to catch all panics, see the “Notes” section below.
///
/// Run `p01_panic_vs_error` to demonstrate what this does.
pub fn panic_vs_error() {
    // Technically you can catch a panic, but it is not recommended to do so. This mechanism is used
    // for special cases such as Rust code that is being called from C.
    match catch_unwind(|| panic!("Oh no")) {
        Ok(()) => println!("catch_unwind succeeded"),
        Err(_) => println!("catch_unwind encountered a panic"),
    }
}

/// ## Getting Help from the Compiler (p. 156)
///
/// This is a side note, but sometimes is required that you know what type a function is returning
/// and it's not easy to figure it out. This can happen when using `Sanfu`'s `source` field, for
/// example.
///
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
        // Uncomment this and buld `lib.rs` to demonstrate.
        //
        // let iter: () = what_type_does_this_return();
    }
}

/// # Creating an Error from Scratch
/// We typically use libraries like `Snafu`, but let's make an error from scratch.
mod custom_error {
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    /// Technically speaking, anything can be used as an error. This is non idiomatic, though,
    /// because callers expect the error type to implement `Error`. `String` does *not* implement
    /// `Error`.
    ///
    /// Why?
    ///
    /// Probably because stringly-typed programming is considered harmful. See `Go` errors for an
    /// example of this. `Go` programmers often have to parse strings to handle errors.
    fn dumb_error_type() -> Result<(), String> {
        Err(String::from("Oh no!"))
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

        /// The underlying error that is being wrapped by `BetterError` (if present). Note that it
        /// is very common and idiomatic to require error types to also implement `Send` and `Sync`.
        /// Without `Send` and `Sync` users will struggle to use your library in `async` or multi-
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
            // This is ugly and took me a long time to figure out, but we are just casting our
            // `source` to the type that is defined by the `Error` trait.
            self.source.as_ref().map(|e| e.as_ref() as &(dyn Error))
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    /// An even better Error would give the user some enum to explain the error condition. It is
    /// idiomatic to call this `Kind` or `ErrorKind` after `std::io::ErrorKind`.
    ///
    /// https://doc.rust-lang.org/std/io/enum.ErrorKind.html
    #[derive(Debug, Clone, Copy)]
    pub enum Kind {
        /// The universe collapsed.
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

    /// Here is a function that returns a `BestError` so we can see how to use it.
    fn simulate_universe() -> std::result::Result<(), BestError> {
        // Try to do something that results in an error.
        let data = std::fs::read_to_string("/heat-death/signal/file");
        let err = data.err().unwrap();
        // Since we didn't find the heat death signal file, we know the universe imploded.
        Err(BestError {
            message: "The universe experience an implosion situation".to_string(),
            kind: Kind::Implosion,
            source: Some(Box::new(err)),
        })
    }

    /// This function handles an error received from `simulate_universe` by checking the `Kind`
    /// property.
    fn handle_error() {
        match simulate_universe() {
            Ok(_) => {}
            Err(e) => match e.kind {
                Kind::Implosion => {
                    // Restart the universe with a new big bang.
                }
                Kind::HeatDeath => {
                    // Nothing to do.
                }
            },
        }
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
            ErrorTypeTwo
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

/// Next, go to `error_libraries`!
mod go_to_error_libraries {}

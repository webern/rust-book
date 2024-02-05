use chapter_9::helpers::print_program_name;
use chapter_9::mane_error::always_err;
use std::process::ExitCode;

/// # Errors and the `main` Function (p. 164)
///
/// Returning an error from `main` will print the `Debug`-formatted error. This is unfortunate
/// because we usually want to show the user a more nicely formatted error.
///
/// This program demonstrates what we should do to provide a nicely formatted error to the user.
///
/// Note: this is what we actually do most of the time but I don't like it becuase I think it is a
/// bad practice to use `std::process::exit`. I prefer to have the conclusion of `main` be the only
/// place that a program can exit. This makes it easier to reason about how the program executes.
///
/// Prefer not to do this:
///
/// ```rust
/// fn main() {
///     match always_err() {
///         Ok(_) => {},
///         Err(e) => {
///             eprintln!("{}", e);
///             // Let's not have our programs exiting in surprising places, even if its in main.
///             std::process::exit(1);
///         }
///     }
/// }
/// ```
///
/// Prefer this:
fn main() -> ExitCode {
    print_program_name();
    match always_err() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}

use chapter_9::helpers::print_program_name;
use chapter_9::mane_error::{always_err, ManeError};

/// # Errors and the `main` Function (p. 164)
///
/// Returning an error from `main` will print the `Debug`-formatted error. This is unfortunate
/// because we usually want to show the user a more nicely formatted error.
///
/// This program prints the `Debug` version of `MainError`
fn main() -> Result<(), ManeError> {
    print_program_name();
    always_err()
}

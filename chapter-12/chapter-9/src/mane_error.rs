use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

// pub fn run_program() -> Result<(), ManeError> {
//     println!("Chapter 9!");
//     panic_vs_error();
//     Ok(())
// }

pub fn always_err() -> std::result::Result<(), ManeError> {
    Err(ManeError)
}

/// This error tells us whether it has been printed with `Debug` or `Display`.
pub struct ManeError;

impl Display for ManeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt("Error: This is the DISPLAY print", f)
    }
}

impl Debug for ManeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt("Error: This is the DEBUG print", f)
    }
}

impl Error for ManeError {}

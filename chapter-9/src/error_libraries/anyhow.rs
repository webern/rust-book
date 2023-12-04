/// `anyhow` is my favorite for error signatures that do not need to be part of the public interface
/// of a Rust library. In other words, this is good for creating error messages in any `private`
/// or `pub(crate)` functions, or in binaries that have no `lib.rs`.
///
/// For `pub` functions in a library, a public-facing type (not `anyhow`) should be used, but
/// `anyhow` is still fine for all of the private code in the library.
///
/// Note that `anyhow` cannot implement `std::error::Error` because of the trickery it uses to
/// ergonomically convert from and `std::error::Error` into itself.
///
use anyhow::{Context, Result};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub struct PublicError {
    kind: PublicErrorKind,
    source: AnyhowError,
}

#[derive(Debug, Clone, Copy)]
pub enum PublicErrorKind {
    /// An error occurred while trying to `Foo` something.
    Foo,
    /// An error occurred while trying to `Bar` something.
    Bar,
}

impl PublicError {
    pub fn kind(&self) -> PublicErrorKind {
        self.kind
    }
}

impl Display for PublicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "A {:?} error occurred: {}", self.kind(), self.source)
    }
}

impl Error for PublicError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

/// Create a little wrapper for `anyhow::Error` to make it impl `std::error::Error`.
struct AnyhowError(anyhow::Error);

impl Debug for AnyhowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // I can switch Display and Debug here because Anyhow is weird!
        Display::fmt(&self.0, f)
    }
}

impl Display for AnyhowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // I can switch Display and Debug here because Anyhow is weird!
        Debug::fmt(&self.0, f)
    }
}

impl std::error::Error for AnyhowError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

/// This shows us that we can use anyhow, but wrap it for our public interface.
pub fn use_anyhow_with_public_error() -> std::result::Result<(), PublicError> {
    let _ = use_anyhow().map_err(|e| PublicError {
        kind: PublicErrorKind::Foo,
        source: AnyhowError(e),
    })?;
    Ok(())
}

/// This shows us how to use `anyhow`.
fn use_anyhow() -> Result<()> {
    let p = PathBuf::from("/very/fake/path/foo/bar");
    let _ = std::fs::read_to_string(&p).context(format!("Unable to read file {}", p.display()))?;
    Ok(())
}

pub fn produce_an_anyhow_error() -> Result<()> {
    use_anyhow()
}

use crate::error_libraries::thiserror::MyError::ErrorOne;
use std::fs;
use std::path::PathBuf;
/// https://hackernoon.com/a-comprehensive-guide-for-handling-errors-in-rust
///
/// > To have cleaner code, let's use thiserror crate. The thiserror crate can help handle dynamic
/// > errors in Rust by allowing the user to define custom error types. It does this through the
/// > #[derive(thiserror::Error)] macro. This macro allows the user to define a custom error type
/// > with a specific set of parameters, such as an error code, a message, and the source of the
/// > error.
///
/// > The user can then use this error type to return an appropriate error value in the event of a
/// > dynamic error. Additionally, the thiserror crate also provides several helpful methods, such
/// > as display_chain, which can be used to chain together multiple errors into a single error
/// > chain.
///
/// My Notes:
///
/// `thiserror` is not "batteries included". It is good for defining an error to be the public-
/// facing error of a library because it does not include the `Context` traits that `Snafu`
/// provides. This means that you will not have any breaking changes from this library, or if you
/// decide to stop using this library. Snafu can cause breaking changes to your public interface,
/// and is a breaking change if you stop using it because it attaches traits to your public error.
///
/// Thus, `thiserror` is little more than a way to eliminate the boilerplace of implementing
/// `Display`, `Error` and (in some cases) `From` traits.
///
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Unable to read file {}: {}", path.display(), source)]
    ErrorOne {
        // Not very good compared to Snafu. I cannot include this field and automatically derive
        // From. And there are no `Context` functions:
        // https://github.com/dtolnay/thiserror/issues/42
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

pub fn use_thiserror() -> Result<(), MyError> {
    let p = PathBuf::from("/bad/path/foo/bar");
    let _ = fs::read_to_string(&p).map_err(|e| ErrorOne { path: p, source: e })?;
    Ok(())
}

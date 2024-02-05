use std::env;

/// This listing shows us how to get command line arguments. The first one is always the name of
/// the program (or the path to the program binary).
///
/// Note: use `std::env::args_os` for non-unicode arguments.
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}

use std::env;

/// In this listing we save the first two arguments into variables. Note that this is not production
/// code, we would not use slice notation to get the args because that would panic without a decent
/// error message.
fn main() {
    let args: Vec<String> = env::args().collect();

    // Bad: these panic if the args do not exist.
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}

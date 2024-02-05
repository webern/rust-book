use std::{env, fs};

/// This listing begins refactoring by moving the parsing out into a function named `parse_config`.
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents =
        fs::read_to_string(file_path).expect(&format!("Unable to read file '{}'", file_path));

    println!("With text:\n{contents}");
}

/// Parsing logic has been refactored out into its own function.
fn parse_config(args: &[String]) -> (&str, &str) {
    // Bad: still a panic for missing arguments.
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}

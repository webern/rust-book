use std::{env, fs};

/// Listing 6 adds a struct for the arguments to make the code more readable.
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

/// A struct has been added which is more readable/usefaul than return a tuple of strings.
struct Config {
    query: String,
    file_path: String,
}

/// Function changed to return a struct with owned `String` fields.
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}

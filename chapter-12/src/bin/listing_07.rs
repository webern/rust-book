use std::{env, fs};

/// Listing 7 uses a constructor for the `Config` struct instead of `parse_config`.
fn main() {
    let args: Vec<String> = env::args().collect();

    // Using a constructor to create an instance of a struct is more idiomatic.
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

/// The `Config` struct now has a constructor.
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    /// What used to be the `parse_config` function is now a constructor for the `Config` struct.
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

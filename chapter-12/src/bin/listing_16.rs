use std::error::Error;
use std::{env, fs, process};

/// Listings 15 and 16 add a test and a failing function. This is "test driven development".
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Now that the `run` function returns a `Result` we need to handle it here.
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// The book has this code in a lib.rs file.
////////////////////////////////////////////////////////////////////////////////////////////////////

/// This function now returns a result.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// This listing adds a failing test
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

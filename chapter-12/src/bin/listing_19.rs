use std::error::Error;
use std::{env, fs, process};

/// Listings 17-19: The search function is now implemented such that the test passes.
/// We have also silenced other println statements, and we are now printing the matching lines like
/// grep does.
///
/// If we run this with `cargo run --bin linsting_19 -- the poem.txt` we get the following lines:
///
/// ```text
/// Then there's a pair of us - don't tell!
/// To tell your name the livelong day
/// ```
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // We will no longer print these so the program will behave more like grep.
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // Now that the `run` function returns a `Result` we need to handle it here.
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// The book has this code in a lib.rs file.
////////////////////////////////////////////////////////////////////////////////////////////////////

/// The main program logic.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // Instead of printing the contents we now print the matches like a grep is supposed to.
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

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

/// The search function is now implemented such that the test passes.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Tests
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

use std::{env, fs, process};

/// Listing 11 extracts program logic from `main` into a function named `run`. Note we use this
/// same naming convention on the Bottlerocket team.
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config)
}

/// This new function extracts the program logic from `main`. Note that this function takes the
/// parsed `Config` as input. This is the *right* way to do this, it makes the `run` function
/// testable.
fn run(config: Config) {
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

/// The `Config` struct now has a constructor that returns a `Result`.
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    /// Note that the authors renamed the function to `build` when they made it return a `Result`.
    /// On the Bottlerocket team we do not do this, we would still call the function `new`.
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

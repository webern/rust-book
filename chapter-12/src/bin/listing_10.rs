use std::{env, fs, process};

/// Listing 9 and 10 go together, they return a `Result` and to the `Config` constructor and handle
/// it at the call site.
fn main() {
    let args: Vec<String> = env::args().collect();

    // Handling a `Result` from the `Config` constructor.
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

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

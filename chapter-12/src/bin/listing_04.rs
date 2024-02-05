use std::{env, fs};

/// We add lines to read the contents of a file `poem.txt`, which I have placed at the root of the
/// Chapter 12 cargo project, and we print its contents.
fn main() {
    let args: Vec<String> = env::args().collect();

    // Bad: these panic if the args do not exist.
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    // New -----------------------------------------------------------------------------------------
    let contents =
        fs::read_to_string(file_path).expect(&format!("Unable to read file '{}'", file_path));

    println!("With text:\n{contents}");
}

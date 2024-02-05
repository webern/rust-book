use std::env;
use std::ffi::OsStr;
use std::path::Path;

fn prog() -> String {
    env::args()
        .next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
        .unwrap()
}

pub fn print_program_name() {
    println!("Chapter 9! - {}", prog());
}

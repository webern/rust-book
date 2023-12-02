use chapter_9::helpers::print_program_name;
use chapter_9::panic_vs_error;

/// This program demonstrates that you can (but shouldn't) catch a panic. (See `lib.rs` for
/// discussion.)
fn main() {
    print_program_name();
    panic_vs_error();
    println!("\n----------------------------------------------");
    println!("The program is still running! It did not exit!")
}

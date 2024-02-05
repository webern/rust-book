use chapter_9::error_libraries::anyhow::{produce_an_anyhow_error, use_anyhow_with_public_error};
use chapter_9::error_libraries::thiserror::use_thiserror;
use chapter_9::helpers::print_program_name;

fn sep() {
    println!();
    println!("-------------------------------------------");
    println!();
}

fn main() {
    print_program_name();
    sep();
    eprintln!("thiserror:\n\n{}", use_thiserror().err().unwrap());
    sep();
    eprintln!(
        "anyhow:\n\n{}",
        use_anyhow_with_public_error().err().unwrap()
    );
    sep();
    println!(
        "Note that `anyhow` is **WEIRD** in that it uses `Debug` to pretty print the error \
    with underlying causes, and `Debug` only prints the top level error message. You **MUST** use \
    `Debug` to see the information you want!"
    );
    sep();
    eprintln!(
        "anyhow Display (yuck! missing information!):\n\n{}",
        produce_an_anyhow_error().err().unwrap()
    );
    sep();
    eprintln!(
        "anyhow Debug (nice):\n\n{:?}",
        produce_an_anyhow_error().err().unwrap()
    );
}

use random_ascii_string::{generate_random_ascii_string, wait_for_specific_chars, Args};
use clap::Parser;
use std::io::{self, Write};

fn main() {
    let mut clipboard = clippers::Clipboard::get();
    let args = Args::parse();

    let result: String = generate_random_ascii_string(&args);
    clipboard.write_text(&result).unwrap();

    print!("Random string saved to clipboard. Show the random string? (y/N): ");
    io::stdout().flush().unwrap();
    let choice = wait_for_specific_chars(&['y', 'Y', 'n', 'N', '\r']);
    if choice == 'y' || choice == 'Y' {
        println!("\n{}", result);
    }
}
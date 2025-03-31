use random_ascii_string::{generate_random_ascii_string, Args};
use clap::Parser;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut clipboard = clippers::Clipboard::get();
    let args = Args::parse();
    let ascii_string = generate_random_ascii_string(&args);
    println!("\n{}", ascii_string);
    clipboard.write_text(ascii_string).unwrap();
    let elapsed = start.elapsed();
    println!("\nElapsed time: {:.2?}", elapsed);
}
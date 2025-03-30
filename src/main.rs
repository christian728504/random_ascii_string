use random_ascii_string::{generate_random_ascii_string, Args};
use clap::Parser;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let args = Args::parse();
    println!("\n{}", generate_random_ascii_string(&args));
    let elapsed = start.elapsed();
    println!("\nElapsed time: {:.2?}", elapsed);
}
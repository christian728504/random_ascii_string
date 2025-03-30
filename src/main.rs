use random_ascii_string::{generate_random_ascii_string, Args};
use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("Random ASCII String: {}", generate_random_ascii_string(&args));
}
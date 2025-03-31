use rand::Rng;
use clap::Parser;
use std::{io, process};

#[derive(Parser, Debug)]
#[command(allow_hyphen_values = true)] 
pub struct Args {
    /// The length of the random string. Length can be between 1 and 2,000,000.
    #[arg(required = true, value_parser = clap::value_parser!(u32).range(1..=2_000_000))]
    length: u32,
    /// Include uppercase letters in the random string.
    #[arg(short, default_value_t = false)]
    uppercase: bool,
    /// Include lowercase letters in the random string.
    #[arg(short, default_value_t = false)]
    lowercase: bool,
    /// Include numbers in the random string.
    #[arg(short, default_value_t = false)]
    numbers: bool,
    /// Include symbols in the random string.
    #[arg(short, default_value_t = false)]
    symbols: bool,

}

impl Args {
    pub fn new(length: u32, uppercase: bool, lowercase: bool, numbers: bool, symbols: bool) -> Self {
        Self {
            length,
            uppercase,
            lowercase,
            numbers,
            symbols,
        }
    }
}

pub fn generate_random_ascii_string (options: &Args) -> String {
    let mut s: String = String::new();
    let result: Vec<char> = get_ascii_chars(options);
    let ascii_chars: Vec<char> = result;
    for _ in 0..options.length {
        let idx: usize = rand::rng().random_range(..ascii_chars.len());
        s.push(ascii_chars[idx]);
    }
    s
}
    
fn get_ascii_chars(options: &Args) -> Vec<char> {
    let mut ascii_chars: Vec<char> = Vec::new();
    if options.uppercase {
        let uppercase: Vec<char> = (65..=90).map(|x| char::from_u32(x as u32).unwrap()).collect();
        ascii_chars.extend(uppercase);
    }
    if options.lowercase {
        let lowercase: Vec<char> = (97..=122).map(|x| char::from_u32(x as u32).unwrap()).collect();
        ascii_chars.extend(lowercase);
    }
    if options.numbers {
        let numbers: Vec<char> = (48..=57).map(|x| char::from_u32(x as u32).unwrap()).collect();
        ascii_chars.extend(numbers);
    }
    if options.symbols {
        let symbols: Vec<char> = vec!['!', '@', '#', '$', '%', '^', '&', '*'];
        ascii_chars.extend(symbols);
    }
    if ascii_chars.is_empty() {
        eprintln!("You must specify at least one ascii type. Run with --help for more info.");
        process::exit(1);
    }
    ascii_chars
}

pub fn wait_for_specific_chars(valid_chars: &[char]) -> char {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("Failed to read line");
    
    let first_char = buffer.chars().next();
    
    match first_char {
        Some(c) if valid_chars.contains(&c) => c,
        _ => valid_chars.last().copied().unwrap_or('N')
    }
}      
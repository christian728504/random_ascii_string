use rand::Rng;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, default_value_t = 12)]
    length: usize,
    #[arg(long, default_value_t = true)]
    uppercase: bool,
    #[arg(long, default_value_t = true)]
    lowercase: bool,
    #[arg(long, default_value_t = true)]
    numbers: bool,
    #[arg(long, default_value_t = true)]
    symbols: bool,

}

impl Args {
    pub fn new(length: usize, uppercase: bool, lowercase: bool, numbers: bool, symbols: bool) -> Self {
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
    let ascii_chars: Vec<char> = get_ascii_chars(options);
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
    ascii_chars
}
        
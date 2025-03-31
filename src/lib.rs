use rand::Rng;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(allow_hyphen_values = true)] 
pub struct Args {
    /// The length of the random string. Length can be between 1 and 2,000,000.
    #[arg(long, default_value_t = 12, value_parser = clap::value_parser!(u32).range(1..=2_000_000))]
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
    let result: Result<Vec<char>, &str> = get_ascii_chars(options);
    if result.is_err() {
        result.err().unwrap().to_string()
    } else {
        let ascii_chars: Vec<char> = result.unwrap();
        for _ in 0..options.length {
            let idx: usize = rand::rng().random_range(..ascii_chars.len());
            s.push(ascii_chars[idx]);
        };
        s
    }
    
    
}

fn get_ascii_chars(options: &Args) -> Result<Vec<char>, &str> {
    let mut ascii_chars: Vec<char> = Vec::new();
    if !(options.uppercase || options.lowercase || options.numbers || options.symbols) {
        return Err("You must choose at least one ascii type. Run with --help for more info.");
    }
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
    Ok(ascii_chars)
}
        
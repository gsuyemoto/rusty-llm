use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("the-verdict.txt")?;
    let reader = BufReader::new(file);

    // Tokenize the contents by white spaces, commas, and periods
    for line in reader.lines() {
        let line = line?;
        for token in tokenize(&line) {
            println!("{}", token);
        }
    }

    Ok(())
}

fn tokenize(line: &str) -> Vec<&str> {
    let mut tokens = Vec::new();

    // Split the line by white spaces, commas, and periods
    for word in line.split_whitespace() {
        tokens.extend(word.split(',').map(|w| w.trim_matches('.')));
    }

    tokens
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("the-verdict.txt")?;

    // Read contents of file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Count chars
    let num_chars = contents.chars().count();
    println!("Number of chars in file: {}", num_chars);

    // Parse file into tokens
    let mut vocabulary: HashMap<_, _> = HashMap::new();
    let mut tokens_all: Vec<String> = Vec::new();
    let mut token = String::new();

    for chr in contents.chars() {
        if chr == '?'
            || chr == '!'
            || chr == '"'
            || chr == ','
            || chr == '.'
            || chr == '('
            || chr == ')'
            || chr == ';'
        {
            tokens_all.push(chr.to_string());
            vocabulary.insert(chr.to_string(), 0);
        } else if chr == ' ' || chr == '\n' {
            tokens_all.push(token.to_string());
            vocabulary.insert(token.to_string(), 0);

            token.clear();
        } else if chr == '-' {
            // check if word or hyphen in token buffer
            if token.len() > 0 {
                // hyphen and then a new hyphen
                if token.contains('-') {
                    token.push(chr);
                    tokens_all.push(token.to_string());
                    vocabulary.insert(token.to_string(), 0);
                    token.clear();
                // word and then hyphen, push word first
                } else {
                    tokens_all.push(token.to_string());
                    vocabulary.insert(token.to_string(), 0);
                    token.clear();
                    token.push(chr);
                }
            } else {
                token.push(chr);
            }
        } else {
            token.push(chr);
        }
    }

    // Show number of tokens
    println!("Number of tokens: {}", vocabulary.len());

    // Show tokens
    let mut index = 0;
    for k in vocabulary.keys() {
        if index == 100 {
            break;
        }
        else { index += 1; }

        println!("First 100 tokens: {k:?}");
    }

    Ok(())
}

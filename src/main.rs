use std::collections::BTreeMap;
// use std::collections::HashMap;
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
    let mut vocabulary: BTreeMap<String, u32> = BTreeMap::new();
    let mut token = String::new();

    for chr in contents.chars() {
        if chr == '?'
            || chr == ':'
            || chr == '!'
            || chr == '"'
            || chr == ','
            || chr == '.'
            || chr == '('
            || chr == ')'
            || chr == ';'
            || chr == '-'
            || chr == ' '
            || chr == '\n'
            || chr == '\''
        {
            if token.len() > 0 {
                token_check(&mut token, &mut vocabulary);
            } else {
                token.push(chr);
                token_check(&mut token, &mut vocabulary);
            }
        } else if chr == '_' {
            if token.len() > 0 {
                token_check(&mut token, &mut vocabulary);
            }
        } else {
            token.push(chr);
        }
    }

    // Show number of tokens
    println!("Number of tokens: {}", vocabulary.len());

    // Show tokens
    let mut index = 0;
    for (key, val) in vocabulary.iter() {
        if index == 300 {
            break;
        } else {
            index += 1;
        }

        println!("First 100 tokens: {} -- {}", key, val);
    }

    Ok(())
}

// fn token_check(token: &mut String, vocab: &mut HashMap<String, u32>) {
fn token_check(token: &mut String, vocab: &mut BTreeMap<String, u32>) {
    if let Some(token_val) = vocab.get_mut(token) {
        *token_val = *token_val + 1;
    } else {
        vocab.insert(token.clone(), 1);
    }

    token.clear();
}

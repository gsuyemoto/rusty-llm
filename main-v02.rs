use std::collections::hash_set::Iter;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};
use std::iter::Enumerate;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Token {
    token: String,
    id: usize,
}

struct Vocabulary {
    all: HashSet<Token>,
    current_id: usize,
}

impl Vocabulary {
    fn new() -> Self {
        Vocabulary {
            all: HashSet::new(),
            current_id: 0,
        }
    }

    fn add(&mut self, token: String) {
        // Try to add token to vocabulary
        // and increment id if successful
        if self.all.insert(Token {
            token,
            id: self.current_id,
        }) {
            self.current_id += 1;
        }
    }

    fn size(&self) -> usize {
        self.current_id
    }

    fn iter_enum(&self) -> Enumerate<Iter<'_, Token>> {
        self.all.iter().enumerate()
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("the-verdict.txt")?;

    // Read contents of file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Count chars
    let num_chars = contents.chars().count();
    println!("Number of chars in file: {}", num_chars);

    // Parse file into tokens
    let mut vocabulary: Vocabulary = Vocabulary::new();
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
            vocabulary.add(chr.to_string());
        } else if chr == ' ' || chr == '\n' {
            tokens_all.push(token.to_string());
            vocabulary.add(token.to_string());

            token.clear();
        } else if chr == '-' {
            // check if word or hyphen in token buffer
            if token.len() > 0 {
                // hyphen and then a new hyphen
                if token.contains('-') {
                    token.push(chr);
                    tokens_all.push(token.to_string());
                    vocabulary.add(token.to_string());
                    token.clear();
                // word and then hyphen, push word first
                } else {
                    tokens_all.push(token.to_string());
                    vocabulary.add(token.to_string());
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
    println!("Number of tokens: {}", vocabulary.size());

    // Show tokens
    for (i, token) in vocabulary.iter_enum() {
        if i == 100 {
            break;
        }

        println!("First 100 tokens: {token:?}");
    }

    Ok(())
}

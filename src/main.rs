use clap::Parser;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    filename: String,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Token {
    val: String,
    id: u32,
    num_used: u32,
}

impl Token {
    fn new(val: String, id: u32) -> Self {
        Token {
            val,
            id,
            num_used: 0,
        }
    }

    fn inc(&mut self) {
        self.num_used += 1;
    }

    fn id(&mut self, id: u32) {
        self.id = id;
    }
}

struct Vocab {
    tokens: BTreeMap<String, Token>,
    in_order: Vec<Token>,
}

impl Vocab {
    fn new() -> Self {
        Vocab {
            tokens: BTreeMap::new(),
            in_order: Vec::new(),
        }
    }

    fn encode(&mut self, contents: String) {
        let mut token_temp = String::new();
        let mut id = 0u32;

        for chr in contents.chars() {
            // match chr {
            //     c if c.is_alphanumeric() => token_temp.push(chr),
            //     c if token_temp.len() > 0 =>
            // }
            if chr == ' ' {
                println!("adding space");
            }
            if chr.is_alphanumeric() {
                token_temp.push(chr);
            } else {
                if token_temp.len() > 0 {
                    if chr == '-' && token_temp.ends_with('-') {
                        token_temp.pop();
                        self.token_check(&mut "--".to_string(), &mut id);

                        if !token_temp.is_empty() {
                            self.token_check(&mut token_temp, &mut id);
                        }
                    } else if chr == '-' || chr == '\'' {
                        token_temp.push(chr);
                    } else {
                        self.token_check(&mut token_temp, &mut id);
                        self.token_check(&mut chr.to_string(), &mut id);
                        // token_temp.push(chr);
                    }
                } else {
                    self.token_check(&mut chr.to_string(), &mut id);
                }
            }
        }
    }

    fn token_check(&mut self, token: &mut String, id: &mut u32) {
        if let Some(token_val) = self.tokens.get_mut(token) {
            (*token_val).inc();
        } else {
            let new_token = Token::new(token.clone(), *id);
            self.tokens.insert(token.clone(), new_token.clone());
            self.in_order.push(new_token);
        }

        token.clear();
        *id = *id + 1;
    }

    fn show_encoded(&self) {
        println!("All encoded tokens in order {:?}", self.in_order);
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // let mut file = File::open("the-verdict.txt")?;
    let mut file = File::open(args.filename)?;

    // Read contents of file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Count chars
    let num_chars = contents.chars().count();
    println!("Number of chars in file: {}", num_chars);

    // Parse file into tokens
    let mut vocab = Vocab::new();
    vocab.encode(contents);

    // Show number of tokens
    println!("Number of tokens: {}", vocab.tokens.len());

    // Show tokens
    // let mut index = 0;
    // for (key, token) in vocab.tokens.iter_mut() {
    //     token.id(index);
    //     index += 1;

    //     if index < 400 {
    //         println!(
    //             "First 100 tokens: {} -- Times: {} -- ID: {}",
    //             key, token.num_used, token.id
    //         );
    //     }
    // }

    vocab.show_encoded();

    Ok(())
}

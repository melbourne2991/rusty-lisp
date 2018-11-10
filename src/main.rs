use std::env;
use std::fs::File;
use std::io::prelude::*;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filename = &args[1];
        read_file(filename);
    } else {
        println!("No file provided");
    }
}

fn read_file(filename: &String) {
    println!("In file {}", filename);

    let f = File::open(filename).expect("file not found!");
    let mut fbytes = f.bytes();

    let lexer = lexer::Lexer::new(fbytes.by_ref());

    for token in lexer {
        match token {
            lexer::Token::Unexpected => {
                println!("Unexpected token!");
                break;
            }
            _ => println!("{:?}", token),
        }
    }
}

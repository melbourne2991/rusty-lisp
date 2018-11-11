use std::env;
use std::fs::File;
use std::io::prelude::*;

mod compiler;

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
    let lexer = compiler::lexer::Lexer::new(fbytes.by_ref());
    let mut parser = compiler::parser::Parser::new();

    // for (token, metadata) in lexer {
    //     match token {
    //         lexer::TokenType::Unexpected => {
    //             println!("Unexpected token!");
    //             break;
    //         }
    //         _ => {
    //             println!("{:?}", metadata);
    //             println!("{:?}", token);
    //         }
    //     }
    // }
}

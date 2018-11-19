use std::fs::File;
use std::io::prelude::*;
use std::thread;

mod errors;
mod file_tree;
mod parse_tree;

pub mod lexer;
pub mod parser;
pub mod source_ast;

pub fn parse_tree_from_file(filename: String) -> (String, parse_tree::PTree) {
  let f = File::open(&filename).expect("file not found!");

  let mut fbytes = f.bytes();
  let mut parser = parser::Parser::new();

  let lexer = lexer::Lexer::new(fbytes.by_ref());

  for token in lexer {
    parser.feed(token)
  }

  let parse_tree = parser.tree;

  (filename, parse_tree)
}

pub fn file_tree(filenames: Vec<&String>) -> file_tree::FTree {
  let mut results = vec![];

  for filename in filenames {
    let thread_name = format!("File parser for: {}", filename);
    let cloned_filename = filename.clone();

    let t = thread::Builder::new()
      .name(thread_name)
      .spawn(move || parse_tree_from_file(cloned_filename));

    let result = t
      .expect("Problem spawning file parser thread")
      .join()
      .expect("Child file parser thread panicked");

    results.push(result);
  }

  file_tree::FTree::new(results)
}

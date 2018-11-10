mod lexer;

pub struct Parser<L> {
  lexer: L
}

impl<L: Iterator<Item = lexer::Token>> Parser {
  pub fn new(lexer: L) {
    let parser = Parser {
      lexer
    }
  }
}
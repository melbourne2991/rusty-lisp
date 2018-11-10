mod lexer;

pub struct Parser<L> {
  lexer: L
}

impl<L: Iterator<Item = lexer::TokenType>> Parser {
  pub fn new(lexer: L) {
    let parser = Parser {
      lexer
    }
  }
}
use compiler::lexer::{TokenMetadata, TokenType};

enum NonTerminal {
  Program,
}

pub struct Parser<L: Iterator<Item = (TokenType, TokenMetadata)>> {
  lexer: L,
}

impl<L: Iterator<Item = (TokenType, TokenMetadata)>> Parser<L> {
  pub fn new(lexer: L) -> Parser<L> {
    let parser = Parser { lexer: lexer };

    parser
  }

  pub fn parse(&mut self) {
    for (token, metadata) in &mut self.lexer {
      match token {
        TokenType::Unexpected => {
          println!("Unexpected token!");
          break;
        }
        _ => {
          println!("{:?}", metadata);
          println!("{:?}", token);
        }
      }
    }
  }
}

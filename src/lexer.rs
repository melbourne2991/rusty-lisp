use std::io::Error;

#[derive(Debug)]
pub enum Token {
  LeftParen,
  RightParen,
  Name(String),
  Whitespace,
  Unexpected,
}

pub struct Lexer<R> {
  chars: R,
  current: Option<char>,
}

impl<R: Iterator<Item = Result<u8, Error>>> Lexer<R> {
  pub fn new(it: R) -> Lexer<R> {
    let lexer = Lexer {
      chars: it,
      current: None,
    };

    lexer
  }

  pub fn read_name(&mut self) -> Token {
    let mut name = String::new();

    while let Some(c) = self.current {
      match c {
        c if c.is_ascii_alphabetic() => {
          self.current = Some(c);
          name.push(c)
        }
        _ => break,
      }

      self.next_byte();
    }

    Token::Name(name)
  }

  pub fn read_token(&mut self) -> Option<Token> {
    let c = self.next_byte();

    match c {
      Some(c) => match c {
        '(' => Some(Token::LeftParen),
        ')' => Some(Token::RightParen),
        '"' => Some(self.read_str()),
        c if c.is_ascii_alphabetic() => Some(self.read_name()),
        c if c.is_ascii_whitespace() => Some(Token::Whitespace),

        _ => Some(Token::Unexpected),
      },
      None => None,
    }
  }

  pub fn next_byte(&mut self) -> Option<char> {
    let val = self.chars.next();

    self.current = match val {
      Some(val) => match val {
        Ok(val) => Some(val as char),
        Err(_e) => panic!("Problem reading file"),
      },
      None => None,
    };

    self.current
  }
}

impl<R: Iterator<Item = Result<u8, Error>>> Iterator for Lexer<R> {
  type Item = Token;

  fn next(&mut self) -> Option<Token> {
    self.read_token()
  }
}

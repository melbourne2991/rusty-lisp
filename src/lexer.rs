use std::io::Error;

#[derive(Debug)]
pub enum Token {
  LeftParen,
  RightParen,
  LeftSqrParen,
  RightSqrParen,
  Str(String),
  Name(String),
  Symbol(String),
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

    while let Some(c) = self.next_byte() {
      match c {
        c if c.is_ascii_alphabetic() => {
          name.push(c);
          self.consume_current();
        }
        _ => break,
      }
    }

    Token::Name(name)
  }

  pub fn read_symbol(&mut self) -> Token {
    let mut name = String::new();

    self.consume_current();

    while let Some(c) = self.next_byte() {
      match c {
        c if c.is_ascii_alphabetic() => {
          name.push(c);
          self.consume_current();
        }
        _ => break,
      }
    }

    Token::Symbol(name)
  }

  pub fn read_str(&mut self) -> Token {
    let mut name = String::new();
    let mut esc = false;

    self.consume_current();

    while let Some(c) = self.next_byte() {
      let mut esc_next = false;

      match c {
        '"' if !esc => {
          self.consume_current();
          break;
        }
        '\n' if !esc => return Token::Unexpected,
        '\\' => {
          name.push(c);
          self.consume_current();
          esc_next = true
        }
        _ => {
          name.push(c);
          self.consume_current();
        }
      }

      esc = esc_next;
    }

    Token::Str(name)
  }

  pub fn read_syntax(&mut self, token: Token) -> Option<Token> {
    self.consume_current();
    Some(token)
  }

  pub fn read_token(&mut self) -> Option<Token> {
    let c = self.next_byte();

    match c {
      Some(c) => match c {
        '(' => self.read_syntax(Token::LeftParen),
        ')' => self.read_syntax(Token::RightParen),
        '[' => self.read_syntax(Token::LeftSqrParen),
        ']' => self.read_syntax(Token::RightSqrParen),

        ':' => Some(self.read_symbol()),
        '"' => Some(self.read_str()),

        c if c.is_ascii_alphabetic() => Some(self.read_name()),
        c if c.is_ascii_whitespace() => self.read_syntax(Token::Whitespace),

        _ => Some(Token::Unexpected),
      },
      None => None,
    }
  }

  pub fn consume_current(&mut self) {
    self.current = None;
  }

  pub fn next_byte(&mut self) -> Option<char> {
    let current = self.current;

    match current {
      None => {
        self.current = match self.chars.next() {
          Some(val) => match val {
            Ok(val) => Some(val as char),
            Err(_e) => panic!("Problem reading file"),
          },
          None => None,
        };

        self.current
      }
      Some(current) => Some(current),
    }
  }
}

impl<R: Iterator<Item = Result<u8, Error>>> Iterator for Lexer<R> {
  type Item = Token;

  fn next(&mut self) -> Option<Token> {
    self.read_token()
  }
}

use std::io::Error;

#[derive(Debug, Clone)]
pub enum TokenType {
  LeftParen,
  RightParen,
  LeftSqrParen,
  RightSqrParen,
  Whitespace,
  Unexpected,
  NewLine,
  Str(String),
  Name(String),
  Symbol(String),
}

#[derive(Debug, Copy, Clone)]
pub struct TokenMetadata {
  start_line: u16,
  start_col: u16,
  end_line: u16,
  end_col: u16,
}

#[derive(Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub metadata: TokenMetadata,
}

impl Clone for Token {
  fn clone(&self) -> Self {
    Token {
      token_type: self.token_type.clone(),
      metadata: self.metadata,
    }
  }
}

pub struct Lexer<R> {
  chars: R,
  current: Option<char>,
  line: u16,
  col: u16,
}

impl<R: Iterator<Item = Result<u8, Error>>> Lexer<R> {
  pub fn new(it: R) -> Lexer<R> {
    let lexer = Lexer {
      chars: it,
      current: None,
      line: 1,
      col: 1,
    };

    lexer
  }

  pub fn read_name(&mut self) -> TokenType {
    let mut name = String::new();

    while let Some(c) = self.current_byte() {
      match c {
        c if c.is_ascii_alphabetic() => {
          name.push(c);
          self.consume_current();
        }
        _ => break,
      }
    }

    TokenType::Name(name)
  }

  pub fn read_symbol(&mut self) -> TokenType {
    let mut name = String::new();

    self.consume_current();

    while let Some(c) = self.current_byte() {
      match c {
        c if c.is_ascii_alphabetic() => {
          name.push(c);
          self.consume_current();
        }
        _ => break,
      }
    }

    TokenType::Symbol(name)
  }

  pub fn read_str(&mut self) -> TokenType {
    let mut name = String::new();
    let mut esc = false;
    let mut closed = false;

    self.consume_current();

    while let Some(c) = self.current_byte() {
      let mut esc_next = false;

      match c {
        '"' if !esc => {
          closed = true;
          self.consume_current();
          break;
        }
        '\n' if !esc => return TokenType::Unexpected,
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

    if closed {
      return TokenType::Str(name);
    }

    TokenType::Unexpected
  }

  pub fn read_syntax(&mut self, token: TokenType) -> Option<TokenType> {
    self.consume_current();
    Some(token)
  }

  pub fn read_token(&mut self) -> Option<TokenType> {
    let c = self.current_byte();

    match c {
      Some(c) => match c {
        '(' => self.read_syntax(TokenType::LeftParen),
        ')' => self.read_syntax(TokenType::RightParen),
        '[' => self.read_syntax(TokenType::LeftSqrParen),
        ']' => self.read_syntax(TokenType::RightSqrParen),

        ':' => Some(self.read_symbol()),
        '"' => Some(self.read_str()),

        c if c.is_ascii_alphabetic() => Some(self.read_name()),

        c if c.is_ascii_whitespace() => match c {
          '\n' => {
            self.consume_current();

            self.line = self.line + 1;
            self.col = 1;

            Some(TokenType::NewLine)
          }
          _ => self.read_syntax(TokenType::Whitespace),
        },

        _ => Some(TokenType::Unexpected),
      },
      None => None,
    }
  }

  pub fn consume_current(&mut self) {
    self.col = self.col + 1;
    self.current = None;
  }

  pub fn current_byte(&mut self) -> Option<char> {
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
    let start_line = self.line;
    let start_col = self.col;

    let token_type = self.read_token();

    match token_type {
      Some(token_type) => Some(Token {
        token_type: token_type,
        metadata: TokenMetadata {
          start_line: start_line,
          start_col: start_col,
          end_line: self.line,
          end_col: self.col,
        },
      }),
      None => None,
    }
  }
}

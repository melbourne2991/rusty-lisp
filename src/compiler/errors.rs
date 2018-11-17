use compiler::lexer::TokenType;
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct UnexpectedTokenError {
  token_type: TokenType,
}

impl UnexpectedTokenError {
  pub fn new(token_type: TokenType) -> UnexpectedTokenError {
    UnexpectedTokenError { token_type }
  }
}

impl fmt::Display for UnexpectedTokenError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Unexpected token :: {}", self.token_type)
  }
}

impl error::Error for UnexpectedTokenError {
  fn description(&self) -> &str {
    "Unexpected token"
  }

  fn cause(&self) -> Option<&error::Error> {
    // Generic error, underlying cause isn't tracked.
    None
  }
}

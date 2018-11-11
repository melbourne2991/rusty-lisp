use compiler::lexer::{TokenMetadata, TokenType};

enum NonTerminal {
  Program,
}

enum ASTNodeType<'a> {
  Terminal((TokenType, TokenMetadata)),
  NonTerminal(ASTNonTerminal<'a>),
}

struct ASTNonTerminal<'a> {
  node_type: NonTerminal,
  children: &'a Vec<ASTNodeType<'a>>,
}

struct ASTTree<'a> {
  nodes: Vec<ASTNonTerminal<'a>>,
}

#[derive(Debug, Copy, Clone)]
enum ParserState {
  List,
}

pub struct Parser<'a> {
  tree: ASTTree<'a>,
  token: Option<(TokenType, TokenMetadata)>,
  state: Vec<ParserState>,
}

impl<'a> Parser<'a> {
  pub fn new() -> Parser<'a> {
    let parser = Parser {
      tree: ASTTree { nodes: vec![] },
      token: None,
      state: vec![],
    };

    parser
  }

  pub fn feed(&mut self, token: (TokenType, TokenMetadata)) {
    match self.current_state() {
      None => self.handle_default_state(token),
      _ => {}
    }
  }

  fn current_state(&mut self) -> Option<ParserState> {
    self.state.first().cloned()
  }

  fn handle_default_state(&mut self, (token_type, metadata): (TokenType, TokenMetadata)) {
    match token_type {
      TokenType::Whitespace => {}
      TokenType::LeftParen => self.state.push(ParserState::List),
      _ => panic!("Invalid token"),
    }
  }
}

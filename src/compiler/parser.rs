use compiler::lexer::{TokenMetadata, TokenType};

#[derive(Clone, Copy)]
enum NonTerminal {
  Program,
  List,
}

enum ASTNodeType {
  Terminal((TokenType, TokenMetadata)),
  NonTerminal(ASTNonTerminal),
}

struct ASTNonTerminal {
  node_type: NonTerminal,
  children: Vec<ASTNodeType>,
}

struct ASTTree {
  nodes: Vec<ASTNonTerminal>,
}

pub struct Parser {
  tree: ASTTree,
  token: Option<(TokenType, TokenMetadata)>,
  state: Vec<(NonTerminal, ASTNonTerminal)>,
}

impl ASTNonTerminal {
  pub fn new(node_type: NonTerminal) -> ASTNonTerminal {
    ASTNonTerminal {
      node_type: node_type,
      children: vec![],
    }
  }
}

impl Parser {
  pub fn new() -> Parser {
    let parser = Parser {
      tree: ASTTree { nodes: vec![] },
      token: None,
      state: vec![],
    };

    parser
  }

  pub fn feed(&mut self, token: (TokenType, TokenMetadata)) {
    if let Some(current_state) = self.current_state() {
      match current_state {
        NonTerminal::Program => self.handle_default_state(token),
        NonTerminal::List => {}
        _ => {}
      };
    }
  }

  fn new_parser_state(&mut self, node_type: NonTerminal) {
    let node = ASTNonTerminal::new(node_type);
  }

  fn current_state(&self) -> Option<NonTerminal> {
    if let Some(&(non_terminal, _)) = self.state.first() {
      return Some(non_terminal);
    }

    None
  }

  fn handle_default_state(&mut self, (token_type, metadata): (TokenType, TokenMetadata)) {
    match token_type {
      TokenType::Whitespace => {}
      TokenType::LeftParen => self.state.push((
        NonTerminal::List,
        ASTNonTerminal {
          node_type: NonTerminal::List,
          children: vec![],
        },
      )),
      _ => panic!("Invalid token"),
    }
  }
}

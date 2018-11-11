use compiler::lexer::{TokenMetadata, TokenType};

enum NonTerminal {
  Program,
  List,
}

enum ASTNodeType<'a> {
  Terminal((TokenType, TokenMetadata)),
  NonTerminal(ASTNonTerminal<'a>),
}

struct ASTNonTerminal<'a> {
  node_type: NonTerminal,
  children: Vec<&'a ASTNodeType<'a>>,
}

struct ASTTree<'a> {
  nodes: Vec<ASTNonTerminal<'a>>,
}

pub struct Parser<'a> {
  tree: ASTTree<'a>,
  token: Option<(TokenType, TokenMetadata)>,
  state: Vec<(NonTerminal, &'a ASTNonTerminal<'a>)>,
}

impl<'a> ASTNonTerminal<'a> {
  pub fn new(node_type: NonTerminal) -> ASTNonTerminal<'a> {
    ASTNonTerminal {
      node_type: node_type,
      children: vec![],
    }
  }
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
    let current_state = self.state.first();

    match current_state {
      Program => self.handle_default_state(token),
      List => {}
      _ => {}
    };
  }

  fn new_parser_state(&mut self, node_type: NonTerminal) {
    let node = ASTNonTerminal::new(node_type);
  }

  // fn current_state(&self) -> Option<&NonTerminal> {
  //   let state = &self.state;

  //   if let Some((non_terminal, _ast)) = state.first() {
  //     return Some(non_terminal)
  //   }

  //   None
  // }

  fn handle_default_state(&mut self, (token_type, metadata): (TokenType, TokenMetadata)) {
    match token_type {
      TokenType::Whitespace => {}
      TokenType::LeftParen => {}
      _ => panic!("Invalid token"),
    }
  }
}

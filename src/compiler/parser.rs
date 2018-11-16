use compiler::ast_tree::{ASTNode, ASTNodeType, ASTNonTerminal, ASTTree, NonTerminalType};
use compiler::lexer::{Token, TokenMetadata, TokenType};

pub struct Parser {
  token: Option<(TokenType, TokenMetadata)>,
  tree: ASTTree,
  state: Vec<usize>,
}

impl Parser {
  pub fn new() -> Parser {
    let mut tree = ASTTree::new(ASTNonTerminal::new(NonTerminalType::Program, None));
    let root_ref = tree.get_root_ref();

    let parser = Parser {
      tree,
      token: None,
      state: vec![root_ref],
    };

    parser
  }

  pub fn feed(&mut self, token: Token) {
    match self.current_state_type() {
      NonTerminalType::Program => self.handle_default_state(token),
      NonTerminalType::List => {}
    };
  }

  fn new_parser_state(&mut self, non_terminal: ASTNonTerminal) {
    let current_state_ref = self.current_state_ref();

    let node_ref = self
      .tree
      .add_child_to_node(current_state_ref, ASTNode::NonTerminal(non_terminal));

    self.state.push(node_ref);
  }

  fn current_state_ref(&self) -> usize {
    *self.state.last().expect("No current state")
  }

  fn current_state_type(&mut self) -> NonTerminalType {
    let current_state_ref = self.current_state_ref();

    match self.tree.get_node_type(current_state_ref) {
      ASTNodeType::NonTerminal(node_type) => node_type,
      _ => panic!("Current state is not a NonTerminal"),
    }
  }

  fn handle_default_state(&mut self, token: Token) {
    match token.token_type {
      TokenType::Whitespace => {}
      TokenType::LeftParen => self.new_parser_state(ASTNonTerminal::new(
        NonTerminalType::List,
        Some(vec![ASTNode::Terminal(token)]),
      )),
      _ => panic!("Invalid token"),
    }
  }

  fn handle_list_state(&mut self, token: Token) {}
}

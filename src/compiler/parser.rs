use compiler::errors::UnexpectedTokenError;
use compiler::lexer::{Token, TokenType};
use compiler::parse_tree::{NonTerminalType, PTNode, PTNodeType, PTNonTerminal, PTree};
use std::process;

pub struct Parser {
  pub tree: PTree,
  state: Vec<usize>,
}

pub enum ParserAction {
  None,
  UnexpectedToken(Token),
  BeginNonTerminal(NonTerminalType, Token),
  EndNonTerminal(Token),
  Terminal(Token),
}

impl Parser {
  pub fn new() -> Parser {
    let mut tree = PTree::new();

    let root_ref = tree.add_node(&PTNode::NonTerminal(PTNonTerminal::new(
      NonTerminalType::Root,
      None,
    )));

    let parser = Parser {
      tree,
      state: vec![root_ref],
    };

    parser
  }

  pub fn feed(&mut self, token: Token) {
    let parser_action = match self.current_state_type() {
      NonTerminalType::Root => self.handle_default_state(token),
      NonTerminalType::List => self.handle_list_state(token),
    };

    match parser_action {
      ParserAction::None => {}
      ParserAction::Terminal(token) => {
        self.add_terminal_to_current(token);
      }
      ParserAction::UnexpectedToken(token) => {
        print!("Error: {}\n", UnexpectedTokenError::new(token.token_type));
        process::exit(1);
      }
      ParserAction::BeginNonTerminal(non_terminal_type, token) => {
        self.new_parser_state(PTNonTerminal::new(
          non_terminal_type,
          Some(vec![PTNode::Terminal(token)]),
        ));
      }
      ParserAction::EndNonTerminal(token) => {
        self.add_terminal_to_current(token);
        self.state.pop();
      }
    };
  }

  fn new_parser_state(&mut self, non_terminal: PTNonTerminal) {
    let current_state_ref = self.current_state_ref();

    let node_ref = self
      .tree
      .add_to_node(current_state_ref, PTNode::NonTerminal(non_terminal));

    self.state.push(node_ref);
  }

  fn add_terminal_to_current(&mut self, terminal: Token) {
    let current_state_ref = self.current_state_ref();

    self
      .tree
      .add_to_node(current_state_ref, PTNode::Terminal(terminal));
  }

  fn current_state_ref(&self) -> usize {
    *self.state.last().expect("No current state")
  }

  fn current_state_type(&mut self) -> NonTerminalType {
    let current_state_ref = self.current_state_ref();

    match self.tree.get_node_type(current_state_ref) {
      PTNodeType::NonTerminal(node_type) => node_type,
      _ => panic!("Current state is not a NonTerminal"),
    }
  }

  fn handle_default_state(&mut self, token: Token) -> ParserAction {
    match token.token_type {
      TokenType::NewLine => ParserAction::None,
      TokenType::Whitespace => ParserAction::None,
      TokenType::LeftParen => ParserAction::BeginNonTerminal(NonTerminalType::List, token),
      _ => ParserAction::UnexpectedToken(token),
    }
  }

  fn handle_list_state(&mut self, token: Token) -> ParserAction {
    match token.token_type {
      TokenType::NewLine => ParserAction::None,
      TokenType::Whitespace => ParserAction::None,
      TokenType::Symbol(_) => ParserAction::Terminal(token),
      TokenType::Name(_) => ParserAction::Terminal(token),
      TokenType::Str(_) => ParserAction::Terminal(token),
      TokenType::LeftParen => ParserAction::BeginNonTerminal(NonTerminalType::List, token),
      TokenType::RightParen => ParserAction::EndNonTerminal(token),
      _ => ParserAction::UnexpectedToken(token),
    }
  }
}

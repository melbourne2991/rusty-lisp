use compiler::lexer::Token;
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum NonTerminalType {
  Program,
  List,
}

pub enum ASTNodeType {
  Terminal(Token),
  NonTerminal(NonTerminalType),
}

pub enum ASTNode {
  Terminal(Token),
  NonTerminal(ASTNonTerminal),
}

enum ASTNodeInternal {
  Terminal(Token),
  NonTerminal(ASTNonTerminalInternal),
}

struct ASTNonTerminalInternal {
  node_type: NonTerminalType,
  children: Vec<usize>,
}

pub struct ASTNonTerminal {
  node_type: NonTerminalType,
  children: Option<Vec<ASTNode>>,
}

pub struct ASTTree {
  nodes: Vec<ASTNodeInternal>,
}

impl ASTNonTerminal {
  pub fn new(node_type: NonTerminalType, children: Option<Vec<ASTNode>>) -> ASTNonTerminal {
    ASTNonTerminal {
      node_type,
      children,
    }
  }
}

impl ASTTree {
  pub fn new() -> ASTTree {
    let nodes = vec![];
    ASTTree { nodes: nodes }
  }

  pub fn get_node_type(&mut self, node_ref: usize) -> ASTNodeType {
    match self.nodes.get(node_ref).expect("Node not found") {
      ASTNodeInternal::Terminal(token) => ASTNodeType::Terminal(token.clone()),
      ASTNodeInternal::NonTerminal(non_terminal) => {
        ASTNodeType::NonTerminal(non_terminal.node_type)
      }
    }
  }

  pub fn add_node(&mut self, node: &ASTNode) -> usize {
    match node {
      ASTNode::Terminal(token) => self.nodes.push(ASTNodeInternal::Terminal(token.clone())),
      ASTNode::NonTerminal(non_terminal) => {
        let children = &non_terminal.children;

        let new_node = ASTNodeInternal::NonTerminal(ASTNonTerminalInternal {
          node_type: non_terminal.node_type,
          children: match children {
            Some(children) => children.iter().map(|child| self.add_node(child)).collect(),
            _ => vec![],
          },
        });

        self.nodes.push(new_node)
      }
    }

    self.nodes.len() - 1
  }

  pub fn add_to_node(&mut self, node_ref: usize, node: ASTNode) -> usize {
    let child_node_ref = self.add_node(&node);
    let found = self
      .nodes
      .get_mut(node_ref)
      .expect("Node with ref does not exist");

    if let ASTNodeInternal::NonTerminal(result) = found {
      result.children.push(child_node_ref);
      return child_node_ref;
    } else {
      panic!("Node is not a NonTerminal");
    }
  }

  fn traverse_node<F>(&self, node_ref: usize, depth: usize, callback: &mut F)
  where
    F: FnMut(ASTNodeType, usize) -> (),
  {
    if let Some(node) = self.nodes.get(node_ref) {
      match node {
        ASTNodeInternal::NonTerminal(non_terminal) => {
          callback(ASTNodeType::NonTerminal(non_terminal.node_type), depth);

          for child_ref in &non_terminal.children {
            self.traverse_node(*child_ref, depth + 1, callback)
          }
        }
        ASTNodeInternal::Terminal(token) => {
          callback(ASTNodeType::Terminal(token.clone()), depth + 1)
        }
      }
    } else {
      print!("Node not found -> {:?}", node_ref);
    }
  }
}

impl Display for NonTerminalType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      NonTerminalType::Program => write!(f, "Program"),
      NonTerminalType::List => write!(f, "List"),
    }
  }
}

impl Display for ASTTree {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ASTTree: {{\n");

    self.traverse_node(0, 1, &mut |node, depth| match node {
      ASTNodeType::NonTerminal(non_terminal_type) => {
        write!(f, "{:width$}-{}\n", "", non_terminal_type, width = depth);
      }
      ASTNodeType::Terminal(token) => {
        write!(f, "{:width$}-{}\n", "", token, width = depth);
      }
    });

    write!(f, "}}")
  }
}

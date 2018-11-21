use compiler::lexer::{Token, TokenType};
use std::fmt;
use std::fmt::Display;
use std::iter::Map;

#[derive(Clone, Copy, Debug)]
pub enum NonTerminalType {
  Root,
  List,
}

pub enum PTNodeType {
  Terminal(Token),
  NonTerminal(NonTerminalType),
}

pub enum PTNode {
  Terminal(Token),
  NonTerminal(PTNonTerminal),
}

pub enum PTNodeInternal {
  Terminal(Token),
  NonTerminal(PTNonTerminalInternal),
}

pub enum PTDynamicNode<'a> {
  Terminal(Token),
  NonTerminal(PTParentNode<'a>),
}

impl<'a> PTDynamicNode<'a> {
  pub fn as_terminal(&self) -> Token {
    match &self {
      PTDynamicNode::Terminal(token) => token.clone(),
      _ => panic!("Unexpected error"),
    }
  }

  pub fn as_non_terminal(&self) -> PTParentNode<'a> {
    match self {
      PTDynamicNode::NonTerminal(non_terminal) => *non_terminal,
      _ => panic!("Unexpected error"),
    }
  }
}

pub struct PTNonTerminalInternal {
  pub node_type: NonTerminalType,
  children: Vec<usize>,
}

pub struct PTNonTerminal {
  node_type: NonTerminalType,
  children: Option<Vec<PTNode>>,
}

pub struct PTree {
  nodes: Vec<PTNodeInternal>,
}

#[derive(Clone, Copy)]
pub struct PTParentNode<'a> {
  pub internal: &'a PTNonTerminalInternal,
  tree: &'a PTree,
}

impl<'a> PTParentNode<'a> {
  pub fn children_iter(self) -> impl Iterator<Item = PTDynamicNode<'a>> {
    let children = &self.internal.children;

    children
      .into_iter()
      .map(move |child_ref| match self.tree.get_node(*child_ref) {
        PTNodeInternal::Terminal(token) => PTDynamicNode::Terminal(token.clone()),
        PTNodeInternal::NonTerminal(_non_terminal) => {
          PTDynamicNode::NonTerminal(self.tree.get_parent_node(*child_ref))
        }
      })
  }
}

impl PTNonTerminal {
  pub fn new(node_type: NonTerminalType, children: Option<Vec<PTNode>>) -> PTNonTerminal {
    PTNonTerminal {
      node_type,
      children,
    }
  }
}

impl PTree {
  pub const ROOT_NODE_REF: usize = 0;

  pub fn new() -> PTree {
    let nodes = vec![];
    PTree { nodes: nodes }
  }

  pub fn get_node(&self, node_ref: usize) -> &PTNodeInternal {
    self.nodes.get(node_ref).expect("Node not found")
  }

  pub fn get_parent_node<'a>(&'a self, node_ref: usize) -> PTParentNode<'a> {
    PTParentNode {
      internal: self.get_non_terminal(node_ref),
      tree: self,
    }
  }

  pub fn get_terminal(&self, node_ref: usize) -> &Token {
    if let PTNodeInternal::Terminal(token) = self.get_node(node_ref) {
      return token;
    } else {
      panic!("Node was not a token");
    }
  }

  pub fn get_non_terminal(&self, node_ref: usize) -> &PTNonTerminalInternal {
    if let PTNodeInternal::NonTerminal(non_terminal) = self.get_node(node_ref) {
      return non_terminal;
    } else {
      panic!("Node was not a non_terminal");
    }
  }

  pub fn get_node_type(&mut self, node_ref: usize) -> PTNodeType {
    match self.get_node(node_ref) {
      PTNodeInternal::Terminal(token) => PTNodeType::Terminal(token.clone()),
      PTNodeInternal::NonTerminal(non_terminal) => PTNodeType::NonTerminal(non_terminal.node_type),
    }
  }

  pub fn add_node(&mut self, node: &PTNode) -> usize {
    match node {
      PTNode::Terminal(token) => self.nodes.push(PTNodeInternal::Terminal(token.clone())),
      PTNode::NonTerminal(non_terminal) => {
        let children = &non_terminal.children;

        let new_node = PTNodeInternal::NonTerminal(PTNonTerminalInternal {
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

  pub fn add_to_node(&mut self, node_ref: usize, node: PTNode) -> usize {
    let child_node_ref = self.add_node(&node);
    let found = self
      .nodes
      .get_mut(node_ref)
      .expect("Node with ref does not exist");

    if let PTNodeInternal::NonTerminal(result) = found {
      result.children.push(child_node_ref);
      return child_node_ref;
    } else {
      panic!("Node is not a NonTerminal");
    }
  }

  fn traverse_node<F>(&self, node_ref: usize, depth: usize, callback: &mut F)
  where
    F: FnMut(PTNodeType, usize) -> (),
  {
    if let Some(node) = self.nodes.get(node_ref) {
      match node {
        PTNodeInternal::NonTerminal(non_terminal) => {
          callback(PTNodeType::NonTerminal(non_terminal.node_type), depth);

          for child_ref in &non_terminal.children {
            self.traverse_node(*child_ref, depth + 1, callback)
          }
        }
        PTNodeInternal::Terminal(token) => callback(PTNodeType::Terminal(token.clone()), depth + 1),
      }
    } else {
      print!("Node not found -> {:?}", node_ref);
    }
  }
}

// pub struct PTreeChildIterator<'a> {
//   ptree: &'a PTree,
//   non_terminal_iter: std::vec::IntoIter<usize>,
// }

// impl<'a> Iterator for PTreeChildIterator<'a> {
//   type Item = &'a PTNodeInternal;

//   fn next(&mut self) -> Option<&'a PTNodeInternal> {
//     if let Some(node_ref) = self.non_terminal_iter.next() {
//       return Some(self.ptree.get_node(node_ref));
//     }

//     None
//   }
// }

impl Display for NonTerminalType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      NonTerminalType::Root => write!(f, "Root"),
      NonTerminalType::List => write!(f, "List"),
    }
  }
}

impl Display for PTree {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "PTree:\n");

    self.traverse_node(0, 1, &mut |node, depth| match node {
      PTNodeType::NonTerminal(non_terminal_type) => {
        write!(
          f,
          "{:width$}-{}\n",
          "",
          non_terminal_type,
          width = depth + 2
        );
      }
      PTNodeType::Terminal(token) => {
        write!(f, "{:width$}-{}\n", "", token, width = depth + 2);
      }
    });

    write!(f, "")
  }
}

use compiler::lexer::Token;

#[derive(Clone, Copy)]
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
  pub root: usize,
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
  pub fn new(root_node: ASTNonTerminal) -> ASTTree {
    let mut nodes = vec![];
    let root = add_node_to_vec(&mut nodes, &root_node);

    ASTTree { nodes: nodes, root }
  }

  pub fn get_root_ref(&mut self) -> usize {
    self.root
  }

  pub fn get_node_type(&mut self, node_ref: usize) -> ASTNodeType {
    match self.nodes.get(node_ref).expect("Node not found") {
      ASTNodeInternal::Terminal(token) => ASTNodeType::Terminal(token.clone()),
      ASTNodeInternal::NonTerminal(non_terminal) => {
        ASTNodeType::NonTerminal(non_terminal.node_type)
      }
    }
  }

  pub fn add_child_to_node(&mut self, node_ref: usize, child: ASTNode) -> usize {
    let node_refs = self.add_children_to_node(node_ref, vec![child]);
    node_refs[0]
  }

  pub fn add_children_to_node(&mut self, node_ref: usize, children: Vec<ASTNode>) -> Vec<usize> {
    let result = self.nodes.get_mut(node_ref).expect("Node not found");

    if let ASTNodeInternal::NonTerminal(node) = result {
      let node_refs = self.add_nodes(children);

      node.children.extend(node_refs);

      return node_refs.clone();
    } else {
      panic!("Can only add children to NonTerminal nodes")
    }
  }

  fn add_nodes(&mut self, children: Vec<ASTNode>) -> Vec<usize> {
    add_nodes_to_vect(&mut self.nodes, children)
  }

  pub fn add_node(&mut self, non_terminal: ASTNonTerminal) -> usize {
    add_node_to_vec(&mut self.nodes, &non_terminal)
  }

  fn add_leaf(&mut self, token: Token) -> usize {
    add_leaf_to_vec(&mut self.nodes, token)
  }
}

fn add_node_to_vec(vect: &mut Vec<ASTNodeInternal>, non_terminal: &ASTNonTerminal) -> usize {
  let mut children_idxs: Vec<usize> = vec![];

  if let Some(node_children) = non_terminal.children {
    children_idxs = add_nodes_to_vect(vect, node_children);
  }

  vect.push(ASTNodeInternal::NonTerminal(ASTNonTerminalInternal {
    node_type: non_terminal.node_type,
    children: children_idxs,
  }));

  vect.len()
}

fn add_leaf_to_vec(vect: &mut Vec<ASTNodeInternal>, token: Token) -> usize {
  vect.push(ASTNodeInternal::Terminal(token));
  vect.len()
}

fn add_nodes_to_vect(vect: &mut Vec<ASTNodeInternal>, nodes: Vec<ASTNode>) -> Vec<usize> {
  nodes
    .iter()
    .map(|child| match child {
      ASTNode::Terminal(token) => add_leaf_to_vec(vect, token.clone()),
      ASTNode::NonTerminal(non_terminal) => add_node_to_vec(vect, non_terminal),
    }).collect()
}

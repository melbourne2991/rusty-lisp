use compiler::file_tree::FTree;
use compiler::lexer::{Token, TokenType};
use compiler::parse_tree::{NonTerminalType, PTDynamicNode, PTParentNode, PTree};

#[derive(Debug)]
pub enum NodeType {
  AST,
  File,
  Ident,
  Symbol,
  Declaration,
  Expression,
  Str,
}

#[derive(Debug)]
pub enum FileBody {
  Expression(ExpressionNode),
}

#[derive(Debug)]
pub enum DeclarationBody {
  Expression(ExpressionNode),
}

#[derive(Debug)]
pub enum ExpressionArg {
  Expression(ExpressionNode),
  Symbol(SymbolNode),
  Str(StringNode),
  None,
}

#[derive(Debug)]
pub enum ASTBody {
  File(FileNode),
}

#[derive(Debug)]
pub struct StringNode {
  pub node_type: NodeType,
  pub value: String,
}

#[derive(Debug)]
pub struct SymbolNode {
  pub node_type: NodeType,
  pub value: String,
}

#[derive(Debug)]
pub struct IdentNode {
  pub node_type: NodeType,
  pub value: String,
}

#[derive(Debug)]
pub struct ExpressionNode {
  pub node_type: NodeType,
  pub callee: IdentNode,
  pub args: Vec<ExpressionArg>,
}

#[derive(Debug)]
pub struct DeclarationNode {
  pub node_type: NodeType,
  pub name: IdentNode,
  pub params: Vec<IdentNode>,
  pub body: Vec<DeclarationBody>,
}

#[derive(Debug)]
pub struct FileNode {
  pub node_type: NodeType,
  pub filename: String,
  pub body: Vec<FileBody>,
}

#[derive(Debug)]
pub struct AST {
  pub node_type: NodeType,
  pub body: Vec<ASTBody>,
}

impl AST {
  pub fn from(file_tree: FTree) -> AST {
    let body_nodes = file_tree.into_iter().map(map_file).collect();

    AST {
      node_type: NodeType::AST,
      body: body_nodes,
    }
  }
}

fn map_file((filename, parse_tree): (String, PTree)) -> ASTBody {
  ASTBody::File(FileNode {
    node_type: NodeType::File,
    filename,
    body: map_file_body(parse_tree),
  })
}

fn map_file_body(parse_tree: PTree) -> Vec<FileBody> {
  let root_node = parse_tree.get_parent_node(PTree::ROOT_NODE_REF);

  root_node
    .children_iter()
    .map(|child| match child {
      PTDynamicNode::Terminal(_token) => panic!("Invalid"),
      PTDynamicNode::NonTerminal(non_terminal) => match non_terminal.internal.node_type {
        NonTerminalType::List => FileBody::Expression(map_expression(non_terminal)),
        _ => panic!("Invalid"),
      },
    }).collect()
}

fn map_ident(token: Token) -> IdentNode {
  match token.token_type {
    TokenType::Name(name) => IdentNode {
      node_type: NodeType::Ident,
      value: name,
    },
    _ => panic!("Invalid: {}", token.token_type),
  }
}

fn map_arg(node: &PTDynamicNode) -> ExpressionArg {
  match node {
    PTDynamicNode::Terminal(token) => match token.clone().token_type {
      TokenType::Str(value) => ExpressionArg::Str(StringNode {
        node_type: NodeType::Str,
        value,
      }),
      TokenType::Symbol(value) => ExpressionArg::Symbol(SymbolNode {
        node_type: NodeType::Symbol,
        value,
      }),
      TokenType::LeftParen => ExpressionArg::None,
      TokenType::RightParen => ExpressionArg::None,
      _ => panic!("Invalid: {}", token.token_type),
    },
    PTDynamicNode::NonTerminal(non_terminal) => match non_terminal.internal.node_type {
      NonTerminalType::List => ExpressionArg::Expression(map_expression(*non_terminal)),
      _ => panic!("Invalid!"),
    },
  }
}

fn map_expression<'a>(non_terminal: PTParentNode<'a>) -> ExpressionNode {
  let children: Box<Vec<PTDynamicNode>> = Box::new(non_terminal.children_iter().collect());

  let first = children.get(1).expect("Unexpected error");
  let token = first.as_terminal();

  let arg_tokens: &[PTDynamicNode] = &children[2..children.len()];
  let arg_nodes = arg_tokens.into_iter().map(|node| map_arg(node)).collect();

  ExpressionNode {
    node_type: NodeType::Expression,
    callee: map_ident(token.clone()),
    args: arg_nodes,
  }
}

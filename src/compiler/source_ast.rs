use compiler::file_tree::FTree;
use compiler::lexer::{Token, TokenType};
use compiler::parse_tree::{NonTerminalType, PTNodeInternal, PTNodeType, PTree};

pub enum NodeType {
  AST,
  File,
  Ident,
  Symbol,
  Declaration,
  Expression,
  Str,
}

pub enum FileBody {
  Expression(ExpressionNode),
}

pub enum DeclarationBody {
  Expression(ExpressionNode),
}

pub enum ExpressionArg {
  Expression(ExpressionNode),
  Symbol(SymbolNode),
  Str(StringNode),
}

pub enum ASTBody {
  File(FileNode),
}

pub struct StringNode {
  pub node_type: NodeType,
  pub value: String,
}

pub struct SymbolNode {
  pub node_type: NodeType,
  pub value: String,
}

pub struct IdentNode {
  pub node_type: NodeType,
  pub value: String,
}

pub struct ExpressionNode {
  pub node_type: NodeType,
  pub callee: IdentNode,
  pub args: Vec<ExpressionArg>,
}

pub struct DeclarationNode {
  pub node_type: NodeType,
  pub name: IdentNode,
  pub params: Vec<IdentNode>,
  pub body: Vec<DeclarationBody>,
}

pub struct FileNode {
  pub node_type: NodeType,
  pub filename: String,
  pub body: Vec<FileBody>,
}

struct AST {
  pub node_type: NodeType,
  pub body: Vec<ASTBody>,
}

impl AST {
  pub fn from(file_tree: FTree) -> AST {
    let body_nodes = file_tree.into_iter().map(map_file);

    AST {
      node_type: NodeType::AST,
      body: vec![],
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
  let root_node = parse_tree.get_root_node();

  parse_tree
    .children_iter(PTree::ROOT_NODE_REF)
    .map(|child| match child {
      PTNodeInternal::Terminal(token) => panic!("Invalid"),
      PTNodeInternal::NonTerminal(non_terminal) => match non_terminal.node_type {
        NonTerminalType::List => {
          let callee_node = parse_tree.get_child(non_terminal, 0);

          // FileBody::Expression(ExpressionNode {
          //   node_type: NodeType::Expression,
          //   callee: map_ident(callee_node),
          //   args:
          // })
        }
        _ => panic!("Invalid"),
      },
    });

  vec![]
}

// fn map_ident(node: &PTNodeInternal) -> IdentNode {
//   match node {
//     PTNodeInternal::Terminal(token) => match token.token_type {
//       TokenType::Name(name) => IdentNode {
//         node_type: NodeType::Ident,
//         value: name,
//       },
//     },
//   }
// }

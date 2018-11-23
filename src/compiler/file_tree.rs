use compiler::parse_tree::PTree;
use std::collections::HashMap;

pub struct FTree {
  pub parse_trees: HashMap<String, PTree>,
}

impl FTree {
  pub fn new(results: Vec<(String, PTree)>) -> FTree {
    let mut parse_trees = HashMap::new();

    for parse_tree_with_filename in results {
      let (filename, result) = parse_tree_with_filename;
      parse_trees.insert(filename, result);
    }

    FTree { parse_trees }
  }
}

impl IntoIterator for FTree {
  type Item = (String, PTree);
  type IntoIter = std::collections::hash_map::IntoIter<String, PTree>;

  fn into_iter(self) -> Self::IntoIter {
    self.parse_trees.into_iter()
  }
}

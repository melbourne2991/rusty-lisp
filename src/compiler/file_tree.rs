use compiler::parse_tree::PTree;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

pub struct FTree {
  pub parse_trees: HashMap<String, PTree>,
}

impl FTree {
  pub fn new(results: Vec<(String, PTree)>) -> FTree {
    let mut parse_trees = HashMap::new();

    for boxed_result in results {
      let (filename, result) = boxed_result;
      parse_trees.insert(filename, result);
    }

    FTree { parse_trees }
  }
}

impl Display for FTree {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "FTree:\n");

    for (filename, ptree) in &self.parse_trees {
      write!(f, "{:width$}-File:{}\n", "", filename, width = 1);
      write!(f, "{:width$}{}\n", "", ptree, width = 2);
    }

    write!(f, "")
  }
}

impl IntoIterator for FTree {
  type Item = (String, PTree);
  type IntoIter = std::collections::hash_map::IntoIter<String, PTree>;

  fn into_iter(self) -> Self::IntoIter {
    self.parse_trees.into_iter()
  }
}

// pub struct FTreeIterator {
//   ftree: FTree
// }

// impl Iterator for FTreeIterator {
//   type Item = (String, PTree);

//   fn next(&mut self) -> Option<(String, PTree)> {

//   }
// }

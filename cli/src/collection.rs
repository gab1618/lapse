use lapse::tree::{Tree, TreeEntry};

pub fn output_tree(level: usize, root: &Tree) {
  let level_spacing = " ".repeat(level);

  for entry in root.iter() {
    match entry {
      TreeEntry::Entry(name) => {
        println!("{}{}", level_spacing, name);
      }
      TreeEntry::Subtree(name, items) => {
        println!("{}{}", level_spacing, name);

        output_tree(level + 1, items);
      }
    }
  }
}

pub fn get_tree_flatlist(tree: &Tree) -> Vec<String> {
  let mut requests: Vec<String> = vec![];

  for entry in tree.iter() {
    match entry {
      TreeEntry::Entry(entry) => {
        requests.push(entry.clone());
      }
      TreeEntry::Subtree(_, items) => {
        let sub_requests = get_tree_flatlist(items);
        for sub in sub_requests {
          requests.push(sub);
        }
      }
    }
  }

  requests
}

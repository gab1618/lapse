use lapse::tree::{FlatTreeConfig, Tree, TreeEntry};

pub fn output_tree(level: usize, root: &Tree, config: FlatTreeConfig) {
  let level_spacing = " ".repeat(level);

  for entry in root.iter() {
    match entry {
      TreeEntry::Entry(name) => {
        if config.files {
          println!("{}{}", level_spacing, name);
        }
      }
      TreeEntry::Subtree(name, items) => {
        if config.dirs {
          println!("{}{}", level_spacing, name);
        }

        output_tree(level + 1, items, config);
      }
    }
  }
}


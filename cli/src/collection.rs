use lapse::tree::{Tree, TreeEntry};

pub fn output_tree(level: usize, root: &Tree, config: FlatlistReadConfig) {
  let level_spacing = " ".repeat(level);

  for entry in root.iter() {
    match entry {
      TreeEntry::Entry(name) => {
        if config.include_files {
          println!("{}{}", level_spacing, name);
        }
      }
      TreeEntry::Subtree(name, items) => {
        if config.include_dirs {
          println!("{}{}", level_spacing, name);
        }

        output_tree(level + 1, items, config);
      }
    }
  }
}

#[derive(Clone, Copy, Default)]
pub struct FlatlistReadConfig {
  pub include_files: bool,
  pub include_dirs: bool,
}

impl FlatlistReadConfig {
  pub fn files(mut self, value: bool) -> Self {
    self.include_files = value;
    self
  }

  pub fn dirs(mut self, value: bool) -> Self {
    self.include_dirs = value;
    self
  }
}

pub fn get_tree_flatlist(tree: &Tree, config: FlatlistReadConfig) -> Vec<String> {
  let mut entries: Vec<String> = vec![];

  for entry in tree.iter() {
    match entry {
      TreeEntry::Entry(entry) => {
        if config.include_files {
          entries.push(entry.clone());
        }
      }
      TreeEntry::Subtree(entry, items) => {
        if config.include_dirs {
          entries.push(entry.clone());
        }
        let sub_requests = get_tree_flatlist(items, config);
        for sub in sub_requests {
          entries.push(sub);
        }
      }
    }
  }

  entries
}

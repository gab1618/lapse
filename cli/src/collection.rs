use lapse::tree::{FlatTreeConfig, TraverseEntryKind, Tree};

pub fn output_tree(root: &Tree, config: FlatTreeConfig) {
  root.traverse(0, &|entry| {
    let depth_spacing = " ".repeat(entry.depth);
    match entry.kind {
      TraverseEntryKind::Entry => {
        if config.files {
          println!("{}{}", depth_spacing, entry.name);
        }
      }
      TraverseEntryKind::Subtree => {
        if config.dirs {
          println!("{}{}", depth_spacing, entry.name);
        }
      }
    }
  });
}

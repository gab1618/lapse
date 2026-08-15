use lapse::tree::{FlatTreeConfig, TraverseEntryKind, Tree, resource::Resource};

use crate::{command::open_lapse, select::select_tree_entry};

fn output_env_tree(root: &Tree, current_env: String) {
  root.traverse(Default::default(), 0, &|entry| {
    let depth_spacing = " ".repeat(entry.depth);
    match entry.kind {
      TraverseEntryKind::Entry => {}
      TraverseEntryKind::Subtree => {
        let marker = if current_env == entry.name { "*" } else { "" };
        println!("{}{}{}", marker, depth_spacing, entry.name);
      }
    }
  });
}

pub fn ls(path: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Env, path)?;

  let current_env = lapse.current_env();

  output_env_tree(&tree, current_env);
  Ok(())
}
pub fn switch(name: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Env, None)?;

  let flatlist_config = FlatTreeConfig::default()
    .include_dirs(true)
    .include_files(false);
  let seleced_env = select_tree_entry(&tree, name, flatlist_config)?;

  lapse.switch_env(&seleced_env)?;
  println!("Switched to env: {}", seleced_env);

  Ok(())
}

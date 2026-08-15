use lapse::tree::{FlatTreeConfig, resource::Resource};

use crate::{collection::output_tree, command::open_lapse, select::select_tree_entry};

pub fn ls(path: Option<String>) -> crate::Result<()> {
  // TODO: mark the current env you are in
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Env, path)?;

  let flatlist_config = FlatTreeConfig::default()
    .include_dirs(true)
    .include_files(false);
  output_tree(&tree, flatlist_config);
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

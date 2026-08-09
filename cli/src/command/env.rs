use lapse::tree::resource::Resource;

use crate::{
  collection::{FlatlistReadConfig, output_tree},
  command::open_lapse,
  select::select_tree_entry,
};

pub fn ls(path: Option<String>) -> crate::Result<()> {
  // TODO: mark the current env you are in
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Env, path)?;

  let flatlist_config = FlatlistReadConfig::default().dirs(true);
  output_tree(0, &tree, flatlist_config);
  Ok(())
}
pub fn switch(name: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Env, None)?;

  let flatlist_config = FlatlistReadConfig::default().dirs(true);
  let seleced_env = select_tree_entry(&tree, name, flatlist_config)?;

  lapse.switch_env(&seleced_env)?;
  println!("Switched to env: {}", seleced_env);

  Ok(())
}

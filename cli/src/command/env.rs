use lapse::tree::resource::Resource;

use crate::{collection::output_tree, command::open_lapse, select::select_tree_entry};

pub fn ls(path: Option<String>) -> crate::Result<()> {
  // TODO: mark the current env you are in
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Env, path)?;
  output_tree(0, &tree);
  Ok(())
}
pub fn switch(name: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Env, None)?;
  let seleced_env = select_tree_entry(&tree, name)?;

  lapse.switch_env(&seleced_env)?;
  println!("Switched to env: {}", seleced_env);

  Ok(())
}

use lapse::tree::resource::Resource;

use crate::{
  collection::{FlatlistReadConfig, output_tree},
  command::open_lapse,
  select::select_tree_entry,
};

pub async fn run(script: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Scripts, None)?;

  let flatlist_config = FlatlistReadConfig::default().files(true);
  let selected_script = select_tree_entry(&tree, script, flatlist_config)?;

  lapse.run_script(&selected_script).await?;

  Ok(())
}

pub fn ls(path: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Scripts, path)?;

  let flatlist_config = FlatlistReadConfig::default().files(true).dirs(true);
  output_tree(0, &tree, flatlist_config);

  Ok(())
}

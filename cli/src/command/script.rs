use lapse::tree::{FlatTreeConfig, resource::Resource};

use crate::{collection::output_tree, command::open_lapse, select::select_tree_entry};

pub async fn run(script: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Scripts, None)?;

  let selected_script = select_tree_entry(&tree, script, Default::default())?;

  lapse.run_script(&selected_script).await?;

  Ok(())
}

pub fn ls(path: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Scripts, path)?;

  let flatlist_config = FlatTreeConfig::default().include_dirs(true);
  output_tree(&tree, flatlist_config);

  Ok(())
}

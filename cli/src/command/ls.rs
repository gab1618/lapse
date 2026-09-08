use lapse::tree::{FlatTreeConfig, resource::Resource};

use crate::{collection::output_tree, command::open_lapse};

pub fn ls(path: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let collection = lapse.get_resource_tree(Resource::Requests, path)?;

  let flatlist_config = FlatTreeConfig::default().include_dirs(true);
  output_tree(&collection, flatlist_config);

  Ok(())
}

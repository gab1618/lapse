use lapse::tree::resource::Resource;

use crate::{collection::FlatlistReadConfig, command::open_lapse, select::select_tree_entry};

pub async fn send(request: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Requests, None)?;

  let flatlist_config = FlatlistReadConfig::default().files(true);
  let selected_request = select_tree_entry(&tree, request, flatlist_config)?;

  let response = lapse.request(&selected_request).await?;

  print!("{}", response);

  Ok(())
}

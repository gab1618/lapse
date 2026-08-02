use lapse::tree::resource::Resource;

use crate::{command::open_lapse, select::select_tree_entry};

pub fn log(request: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Requests, None)?;

  let selected_request = select_tree_entry(&tree, request)?;

  let log = lapse.get_response_log_raw(&selected_request, 0)?;

  print!("{log}");

  Ok(())
}

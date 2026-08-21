use lapse::tree::resource::Resource;

use crate::{
  command::{log::display::DetailedLogEntry, open_lapse},
  select::select_tree_entry,
};

pub async fn send(request: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Requests, None)?;

  let selected_request = select_tree_entry(&tree, request, Default::default())?;

  let response = lapse.request(&selected_request).await?;

  let formated = DetailedLogEntry(response);

  print!("{}", formated);

  Ok(())
}

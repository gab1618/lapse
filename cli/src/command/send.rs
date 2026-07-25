use lapse::tree::resource::Resource;

use crate::{command::open_lapse, select::select_tree_entry};

pub async fn send(request: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Requests, None)?;

  let selected_request = select_tree_entry(&tree, request)?;

  let response = lapse
    .request(selected_request, lapse.get_eval_ctx()?)
    .await?;
  println!("{}", response.text);

  Ok(())
}

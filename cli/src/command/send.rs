use lapse::{log::ResponseLog, request::runner::RequestRunner, tree::resource::Resource};

use crate::{command::open_lapse, select::select_tree_entry};

pub async fn send(request: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Requests, None)?;

  let selected_request = select_tree_entry(&tree, request)?;

  let runner = RequestRunner::new(lapse.get_eval_ctx()?);

  let request_http = lapse.get_raw_request_http(&selected_request)?;

  let response = runner.execute(&request_http).await?;
  print!("{}", response);

  let log = ResponseLog {
    request: selected_request,
    text: response.text,
    status: response.status,
    headers: response.headers,
  };

  lapse.save_log(&log)?;

  Ok(())
}

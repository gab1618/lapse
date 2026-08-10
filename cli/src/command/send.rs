use lapse::{log::ResponseLog, request::runner::RequestRunner, tree::resource::Resource};

use crate::{collection::FlatlistReadConfig, command::open_lapse, select::select_tree_entry};

pub async fn send(request: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Requests, None)?;

  let flatlist_config = FlatlistReadConfig::default().files(true);
  let selected_request = select_tree_entry(&tree, request, flatlist_config)?;

  let runner = RequestRunner::new(lapse.get_runtime()?, lapse.get_hooks_scripts()?);

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

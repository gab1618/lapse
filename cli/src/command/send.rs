use lapse::{runner::Runner, tree::resource::Resource};

use crate::{
  command::{log::display::DetailedLogEntry, open_lapse},
  select::select_tree_entry,
};

use colored::Colorize;

pub async fn send(request: Option<String>, body_only: bool, dry_run: bool) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let tree = lapse.get_resource_tree(Resource::Requests, None)?;

  let selected_request = select_tree_entry(&tree, request, Default::default())?;

  let runner = Runner::from_space(&lapse)?;
  let raw_req = lapse.get_raw_request_http(&selected_request)?;

  if dry_run {
    let resolved = runner.eval(&raw_req)?;
    print!("{resolved}");
    return Ok(());
  }

  println!("{}", selected_request.blue());
  println!("{raw_req}");

  let confirmed = inquire::Confirm::new("Send request?")
    .with_default(true)
    .prompt()
    .unwrap_or(false);

  if !confirmed {
    return Ok(());
  }

  let response = lapse.request(&selected_request).await?;

  if body_only {
    print!("{}", response.result.text);
    return Ok(());
  }

  let formated = DetailedLogEntry(response);

  print!("{}", formated);

  Ok(())
}

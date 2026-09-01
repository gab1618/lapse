use lapse::tree::resource::Resource;
use lapse_config::Config;

use crate::{command::open_lapse, select::select_tree_entry};

pub fn edit(request: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let reqs = lapse.get_resource_tree(Resource::Requests, None)?;

  let selected = select_tree_entry(&reqs, request, Default::default())?;
  let path = lapse
    .resource_path(Resource::Requests)?
    .join(selected)
    .with_extension("md");

  let config = Config::read();
  let editor = config.get("editor").ok_or(crate::Error::NoEditor)?;

  let mut cmd = std::process::Command::new(editor);
  cmd.arg(&path);

  cmd.status().map_err(crate::Error::EditCommandFail)?;

  Ok(())
}

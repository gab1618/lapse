use lapse::tree::resource::Resource;

use crate::{command::open_lapse, editor::get_editor, select::select_tree_entry};

pub fn edit(request: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let reqs = lapse.get_resource_tree(Resource::Requests, None)?;

  let selected = select_tree_entry(&reqs, request, Default::default())?;
  let path = lapse
    .resource_path(Resource::Requests)?
    .join(selected)
    .with_extension("md");

  let editor = get_editor()?;

  let mut cmd = std::process::Command::new(editor);
  cmd.arg(&path);

  cmd.status().map_err(crate::Error::EditCommandFail)?;

  Ok(())
}

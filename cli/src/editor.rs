use lapse_config::Config;

pub fn get_editor() -> crate::Result<String> {
  let config = Config::read();

  let editor = config.get("editor").ok_or(crate::Error::NoEditor)?;

  Ok(editor.to_string())
}

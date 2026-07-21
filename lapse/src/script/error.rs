#[derive(Debug, thiserror::Error)]
pub enum ScriptError {
  #[error("Could not read script: {0}")]
  ReadScriptFile(#[source] std::io::Error),
}

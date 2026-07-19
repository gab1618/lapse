#[derive(Debug, thiserror::Error)]
pub enum TreeError {
  #[error("Could not read dir: {0}")]
  ReadDir(#[source] std::io::Error),
  #[error("Could not parse tree path")]
  ParseTreePath,
}

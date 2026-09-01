use crate::command::log::error::LogError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Lapse(#[from] lapse::Error),
  #[error(transparent)]
  Log(#[from] LogError),
  #[error(transparent)]
  Template(#[from] lapse_template::Error),
  #[error("Could not get current dir: {0}")]
  GetCurrentDir(#[source] std::io::Error),
  #[error("Could not invoke prompt: {0}")]
  InvokePrompt(#[source] inquire::InquireError),
  #[error("No resource matched query: {0}")]
  NoResourceMatch(String),
  #[error("No configured editor found")]
  NoEditor,
  #[error("Failed edit command: {0}")]
  EditCommandFail(#[source] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Lapse(#[from] lapse::Error),
  #[error("Could not get current dir: {0}")]
  GetCurrentDir(#[source] std::io::Error),
  #[error("Could not invoke prompt: {0}")]
  InvokePrompt(#[source] inquire::InquireError),
}

pub type Result<T> = std::result::Result<T, Error>;

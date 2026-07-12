use crate::env::error::EnvError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Could not create subdir: {0}")]
  CreateSpaceDir(String),
  #[error(transparent)]
  Env(#[from] EnvError),
}

pub type Result<T> = std::result::Result<T, Error>;

use crate::{env::error::EnvError, state::error::StateError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Could not create subdir: {0}")]
  CreateSpaceDir(String),
  #[error(transparent)]
  Env(#[from] EnvError),
  #[error(transparent)]
  State(#[from] StateError),
}

pub type Result<T> = std::result::Result<T, Error>;

use crate::{env::error::EnvError, eval::error::EvalError, state::error::StateError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Could not create subdir: {0}")]
  CreateSpaceDir(String),
  #[error(transparent)]
  Env(#[from] EnvError),
  #[error(transparent)]
  State(#[from] StateError),
  #[error(transparent)]
  Eval(#[from] EvalError),
}

pub type Result<T> = std::result::Result<T, Error>;

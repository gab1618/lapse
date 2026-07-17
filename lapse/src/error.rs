use crate::{
  env::error::EnvError, eval::error::EvalError, log::error::LogError, state::error::StateError,
};

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
  #[error(transparent)]
  Log(#[from] LogError),
}

pub type Result<T> = std::result::Result<T, Error>;

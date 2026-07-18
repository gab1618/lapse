use crate::{env::error::EnvError, log::error::LogError, state::error::StateError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Could not create subdir: {0}")]
  CreateSpaceDir(String),
  #[error("Could not open sample request file: {0}")]
  OpenSampleFile(#[source] std::io::Error),
  #[error("Could not write sample file: {0}")]
  WriteSampleFile(#[source] std::io::Error),
  #[error("Could not get parent dir")]
  GetParentDir,
  #[error(transparent)]
  Env(#[from] EnvError),
  #[error(transparent)]
  State(#[from] StateError),
  #[error(transparent)]
  Log(#[from] LogError),
  #[error(transparent)]
  Lua(#[from] mlua::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

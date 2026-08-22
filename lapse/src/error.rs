use crate::{
  env::error::EnvError, log::error::LogError, request::error::RequestError,
  script::error::ScriptError, state::error::StateError, tree::error::TreeError,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Could not create subdir: {0}")]
  CreateSpaceDir(String),
  #[error("Could not open sample request file: {0}")]
  OpenSampleFile(#[source] std::io::Error),
  #[error("Could not write sample file: {0}")]
  WriteSampleFile(#[source] std::io::Error),
  #[error("Could not find lapse space")]
  LapseNotFound,
  #[error(transparent)]
  Env(#[from] EnvError),
  #[error(transparent)]
  State(#[from] StateError),
  #[error(transparent)]
  Log(#[from] LogError),
  #[error(transparent)]
  Lua(#[from] mlua::Error),
  #[error(transparent)]
  Request(#[from] RequestError),
  #[error(transparent)]
  Tree(#[from] TreeError),
  #[error(transparent)]
  Script(#[from] ScriptError),
}

pub type Result<T> = std::result::Result<T, Error>;

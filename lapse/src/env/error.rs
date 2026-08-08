use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
  #[error("Env {0} doesn't exist")]
  NonExistentEnv(String),
  #[error("Could not open variables file: {0}")]
  OpenVariables(#[source] std::io::Error),
  #[error("Could not create env: {0}")]
  Create(#[source] std::io::Error),
  #[error("Could not serialize variables")]
  SerializeVariables,
}

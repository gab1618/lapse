use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
  #[error("Env {0} doesn't exist")]
  NonExistentEnv(String),
  #[error("Could not open env file: {0}")]
  OpenEnvFile(#[source] std::io::Error),
  #[error("Could not parse env")]
  ParseEnv,
  #[error("Could not serialize env")]
  SerializeEnv,
}

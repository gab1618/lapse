use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
  #[error("Env {0} doesn't exist")]
  NonExistentEnv(String),
}

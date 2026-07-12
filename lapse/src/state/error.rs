use thiserror::Error;

#[derive(Debug, Error)]
pub enum StateError {
  #[error("Could not ensure state dir: {0}")]
  EnsureStateDir(#[source] std::io::Error),
  #[error("Could not save state: {0}")]
  SaveState(#[source] std::io::Error),
  #[error("Could not open state: {0}")]
  OpenStateFile(#[source] std::io::Error),
  #[error("Could not read state: {0}")]
  ReadState(#[source] std::io::Error),
}

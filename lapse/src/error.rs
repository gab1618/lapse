#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Could not create subdir: {0}")]
  CreateSpaceDir(String),
}

pub type Result<T> = std::result::Result<T, Error>;

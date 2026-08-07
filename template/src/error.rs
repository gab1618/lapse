#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Could not create template file: {0}")]
  CreateTemplateFile(#[source] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

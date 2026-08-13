use crate::templates::openapi::error::OpenApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Could not create template file: {0}")]
  CreateTemplateFile(#[source] std::io::Error),
  #[error("Could not create collection: {0}")]
  CreateCollection(#[source] std::io::Error),
  #[error(transparent)]
  OpenApi(#[from] OpenApiError),
}

pub type Result<T> = std::result::Result<T, Error>;

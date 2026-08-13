#[derive(Debug, thiserror::Error)]
pub enum OpenApiError {
  #[error("Could not parse schema: {0}")]
  ParseSchema(#[source] serde_yaml::Error),
  #[error("Could not parse schema: missing servers list")]
  NoServerAvailable,
}

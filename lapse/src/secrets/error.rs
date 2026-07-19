#[derive(Debug, thiserror::Error)]
pub enum SecretsError {
  #[error("Could not open secrets file: {0}")]
  OpenSecretsFile(#[source] std::io::Error),
  #[error("Could not parse secrets")]
  ParseSecrets,
}

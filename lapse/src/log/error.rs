#[derive(Debug, thiserror::Error)]
pub enum LogError {
  #[error("Could not ensure logs dir")]
  EnsureLogsDir(#[source] std::io::Error),
  #[error("Could not save log file")]
  SaveLogfile(#[source] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum LogError {
  #[error("Could not ensure logs dir")]
  EnsureLogsDir(#[source] std::io::Error),
  #[error("Could not open log file: {0}")]
  OpenLogFile(#[source] std::io::Error),
  #[error("Could not save log file: {0}")]
  SaveLogfile(#[source] std::io::Error),
  #[error("Could not read log file: {0}")]
  ReadLogFile(#[source] std::io::Error),
}

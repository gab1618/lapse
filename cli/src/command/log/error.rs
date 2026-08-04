#[derive(Debug, thiserror::Error)]
pub enum LogError {
  #[error("Could not send data to pager")]
  SendToPager(#[source] std::fmt::Error),
  #[error("Could not setup pager config: {0}")]
  SetupPagerConfig(minus::error::MinusError),
}

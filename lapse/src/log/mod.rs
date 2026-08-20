pub mod error;
pub mod iter;

use std::{
  fs::{self, OpenOptions},
  time::{SystemTime, UNIX_EPOCH},
};

use crate::{Lapse, log::error::LogError, runner::ExecutionResult};

#[cfg(test)]
mod test;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct ResponseLog {
  pub request: String,
  pub result: ExecutionResult,
}

impl Lapse {
  pub fn save_log(&self, log: &ResponseLog) -> crate::Result<()> {
    // Ensure logs path exists
    fs::create_dir_all(self.logs_path()).map_err(LogError::EnsureLogsDir)?;

    let curr_time = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("time should go forward");

    let filename = curr_time.as_nanos().to_string();

    let full_file_path = self.logs_path().join(filename);

    let f = OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open(full_file_path)
      .map_err(LogError::OpenLogFile)?;

    serde_json::to_writer(f, log).map_err(|_| LogError::SaveLogfile)?;

    Ok(())
  }
}

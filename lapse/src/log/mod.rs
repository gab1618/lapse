pub mod error;

use std::{
  collections::HashMap,
  fmt::Display,
  fs::{self, OpenOptions},
  io::Write,
  time::{SystemTime, UNIX_EPOCH},
};

use crate::{Lapse, log::error::LogError};

pub struct ResponseLog {
  pub request: String,
  pub text: String,
  pub status: u16,
  pub headers: HashMap<String, String>,
}

impl Display for ResponseLog {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{}", self.request)?;
    writeln!(f, "{}", self.status)?;
    for (header, value) in &self.headers {
      writeln!(f, "{}: {}", header, value)?;
    }
    writeln!(f)?;
    writeln!(f, "{}", self.text)
  }
}

impl Lapse {
  pub fn save_log(&self, log: &ResponseLog) -> crate::Result<()> {
    let logs_path = self.logs_path();
    let request_logs_path = logs_path.join(&log.request);

    // Ensure logs path exists
    fs::create_dir_all(&request_logs_path).map_err(LogError::EnsureLogsDir)?;

    let curr_time = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("time should go forward");

    let filename = curr_time.as_millis().to_string();

    let full_file_path = request_logs_path.join(filename);

    let mut f = OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open(full_file_path)
      .map_err(LogError::SaveLogfile)?;

    write!(f, "{log}").unwrap();

    Ok(())
  }
}

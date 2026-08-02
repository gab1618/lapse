pub mod error;

use std::{
  collections::HashMap,
  fmt::Display,
  fs::{self, OpenOptions},
  io::{Read, Write},
  path::PathBuf,
  str::FromStr,
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
    writeln!(f, "{} {}", self.request, self.status)?;
    for (header, value) in &self.headers {
      writeln!(f, "{}: {}", header, value)?;
    }
    writeln!(f)?;
    writeln!(f, "{}", self.text)
  }
}

impl FromStr for ResponseLog {
  type Err = LogError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut lines = s.lines();
    let first = lines.next().unwrap();
    let (request, status) = first.split_once(" ").unwrap();

    let status = u16::from_str(status).unwrap();
    let mut headers = HashMap::new();

    for line in lines.by_ref() {
      if line.is_empty() {
        break;
      }

      let (key, value) = line.split_once(":").unwrap();
      headers.insert(key.trim().to_string(), value.trim().to_string());
    }

    let body_lines = lines.collect::<Vec<&str>>();
    let body = body_lines.join("\n");

    Ok(Self {
      request: request.to_string(),
      text: body,
      status,
      headers,
    })
  }
}

impl ResponseLog {
  pub fn from_read<R: Read>(r: &mut R) -> crate::Result<Self> {
    let mut buf = String::new();
    r.read_to_string(&mut buf).map_err(LogError::ReadLogFile)?;

    Ok(Self::from_str(&buf)?)
  }
}

impl Lapse {
  fn response_logs_path(&self, request: &str) -> PathBuf {
    self.logs_path().join(request)
  }
  pub fn save_log(&self, log: &ResponseLog) -> crate::Result<()> {
    let request_logs_path = self.response_logs_path(&log.request);

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
      .map_err(LogError::OpenLogFile)?;

    write!(f, "{log}").map_err(LogError::SaveLogfile)?;

    Ok(())
  }
  pub fn get_response_log(&self, request: &str, n: usize) -> crate::Result<ResponseLog> {
    let mut all_logs_names = self.get_response_logs_names(request);

    all_logs_names.reverse();

    let entry_name = all_logs_names.get(n).map(String::to_string).unwrap();
    let full_entry_path = self.response_logs_path(request).join(entry_name);

    let mut f = OpenOptions::new().read(true).open(full_entry_path).unwrap();
    let parsed_entry = ResponseLog::from_read(&mut f).unwrap();

    Ok(parsed_entry)
  }
  pub fn get_response_logs_names(&self, request: &str) -> Vec<String> {
    let request_logs_path = self.response_logs_path(request);

    let entries = fs::read_dir(request_logs_path).unwrap();

    let list = entries
      .filter_map(|entry| {
        let resolved = entry.unwrap();

        if resolved.path().is_dir() {
          return None;
        }

        Some(resolved.file_name().to_str().unwrap().to_owned())
      })
      .collect::<Vec<_>>();

    list
  }
}

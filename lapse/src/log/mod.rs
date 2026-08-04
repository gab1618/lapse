pub mod error;

use std::{
  collections::HashMap,
  fmt::Display,
  fs::{self, OpenOptions},
  io::{Read, Write},
  path::PathBuf,
  str::FromStr,
  sync::Arc,
  time::{SystemTime, UNIX_EPOCH},
};

use crate::{Lapse, log::error::LogError};

#[cfg(test)]
mod test;

#[cfg_attr(test, derive(Debug, PartialEq))]
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
    let first = lines.next().ok_or(LogError::ParseHead)?;
    let (request, status) = first.split_once(" ").ok_or(LogError::ParseHead)?;

    let status = u16::from_str(status).map_err(|_| LogError::ParseHead)?;
    let mut headers = HashMap::new();

    for line in lines.by_ref() {
      if line.is_empty() {
        break;
      }

      let (key, value) = line.split_once(":").ok_or(LogError::ParseHeader)?;
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

#[derive(Clone)]
pub struct ResponseLogsIter {
  lapse: Arc<Lapse>,
  request: String,
  src: Vec<String>,
}

impl Iterator for ResponseLogsIter {
  type Item = (String, String);

  fn next(&mut self) -> Option<Self::Item> {
    let name = self.src.pop()?;

    let full_entry_path = self.lapse.response_logs_path(&self.request).join(&name);

    let mut log = OpenOptions::new().read(true).open(full_entry_path).ok()?;

    let mut contents = String::new();

    log.read_to_string(&mut contents).ok()?;

    Some((name, contents))
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

    let filename = curr_time.as_nanos().to_string();

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

  pub fn get_response_logs_names(&self, request: &str) -> crate::Result<Vec<String>> {
    let request_logs_path = self.response_logs_path(request);

    let entries = fs::read_dir(request_logs_path).map_err(LogError::ListLogFiles)?;

    Ok(
      entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| !entry.path().is_dir())
        .filter_map(|entry| {
          let file_name = entry.file_name();
          let str_name = file_name.to_str();

          str_name.map(|inner| inner.to_string())
        })
        .collect::<Vec<_>>(),
    )
  }
  pub fn get_response_log_entry(&self, request: &str, entry_name: &str) -> crate::Result<String> {
    let full_entry_path = self.response_logs_path(request).join(entry_name);

    let content = fs::read_to_string(full_entry_path).map_err(LogError::ReadLogFile)?;

    Ok(content)
  }
  pub fn logs_iter(&self, request: &str) -> crate::Result<ResponseLogsIter> {
    let mut entries_names = self.get_response_logs_names(request)?;

    entries_names.reverse();

    Ok(ResponseLogsIter {
      lapse: Arc::new(self.clone()),
      request: request.to_string(),
      src: entries_names,
    })
  }
}

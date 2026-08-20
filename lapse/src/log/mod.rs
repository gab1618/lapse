pub mod error;
pub mod iter;

use std::{
  collections::HashMap,
  fmt::Display,
  fs::{self, OpenOptions},
  io::{Read, Write},
  str::FromStr,
  time::{SystemTime, UNIX_EPOCH},
};

use mlua::{IntoLua, Lua, Value as LuaValue};

use crate::{Lapse, log::error::LogError};

#[cfg(test)]
mod test;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Default)]
pub struct ResponseLog {
  pub request: String,
  pub text: String,
  pub status: u16,
  pub headers: HashMap<String, String>,
  pub duration: u128,
  pub timestamp: u128,
}

impl Display for ResponseLog {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{}", self.timestamp)?;
    writeln!(f, "{}", self.duration)?;
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
    let raw_timestamp = lines.next().unwrap();
    let raw_duration = lines.next().unwrap();

    let timestamp = u128::from_str(raw_timestamp).unwrap();
    let duration = u128::from_str(raw_duration).unwrap();

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
      duration,
      timestamp,
      text: body,
      status,
      headers,
    })
  }
}

impl IntoLua for ResponseLog {
  fn into_lua(self, lua: &Lua) -> mlua::Result<LuaValue> {
    let table = lua.create_table()?;

    table.set("status", self.status)?;
    table.set("text", self.text)?;
    table.set("headers", self.headers)?;
    table.set("request", self.request)?;
    table.set("duration", self.duration)?;
    table.set("timestamp", self.timestamp)?;

    Ok(LuaValue::Table(table))
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
  pub fn save_log(&self, log: &ResponseLog) -> crate::Result<()> {
    // Ensure logs path exists
    fs::create_dir_all(self.logs_path()).map_err(LogError::EnsureLogsDir)?;

    let curr_time = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("time should go forward");

    let filename = curr_time.as_nanos().to_string();

    let full_file_path = self.logs_path().join(filename);

    let mut f = OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open(full_file_path)
      .map_err(LogError::OpenLogFile)?;

    write!(f, "{log}").map_err(LogError::SaveLogfile)?;

    Ok(())
  }
}

use std::{
  collections::HashMap,
  fs::{self, OpenOptions},
  io::Write,
  path::PathBuf,
};

use reqwest::Client;

pub mod env;
pub mod error;
pub mod eval;
pub mod log;
pub mod parsing;
pub mod request;
pub mod state;

pub use error::{Error, Result};

use crate::{log::ResponseLog, request::RequestFile};

#[cfg(test)]
mod test;

pub struct Lapse {
  path: PathBuf,
}

impl Lapse {
  // TODO: add proper error handling
  pub fn init<P: Into<PathBuf>>(path: P) -> crate::Result<Self> {
    let base_path: PathBuf = path.into();

    let space_dirs = ["requests", "env", ".lapse"];

    space_dirs
      .into_iter()
      .map(|subdir| {
        let dir_path = base_path.join(subdir);
        fs::create_dir_all(dir_path).map_err(|_| Error::CreateSpaceDir(subdir.to_owned()))
      })
      .collect::<Result<Vec<_>>>()?;

    let mut f = OpenOptions::new()
      .create(true)
      .truncate(true)
      .write(true)
      .open(base_path.join("requests").join("request.md"))
      .unwrap();

    let sample_request_contents = include_str!("../assets/request.md");
    f.write_all(sample_request_contents.as_bytes()).unwrap();

    Ok(Self { path: base_path })
  }
  pub fn open<P: Into<PathBuf>>(path: P) -> Self {
    let as_buf: PathBuf = path.into();

    let space_dir_path = as_buf.join(".lapse");
    if !space_dir_path.exists() {
      let parent_path = as_buf.parent().unwrap();
      return Self::open(parent_path);
    }

    Self { path: as_buf }
  }
  fn requests_path(&self) -> PathBuf {
    self.path.join("requests")
  }
  fn logs_path(&self) -> PathBuf {
    self.path.join(".lapse/log")
  }
  fn state_path(&self) -> PathBuf {
    self.path.join(".lapse/state")
  }
  fn env_path(&self) -> PathBuf {
    self.path.join("env")
  }

  pub async fn request(&self, req: &RequestFile, name: String) -> ResponseLog {
    let client = Client::new();

    let request = self.resolve_request(req);
    let parsed_request: reqwest::Request = request.try_into().unwrap();

    let response = client.execute(parsed_request).await.unwrap();

    let mut log_headers = HashMap::new();
    response.headers().iter().for_each(|(name, value)| {
      let str_value = value.to_str().unwrap().to_string();

      log_headers.insert(name.to_string(), str_value);
    });

    let status_code = response.status().as_u16();
    let response_body = response.text().await.unwrap();

    let log = ResponseLog {
      request: name,
      text: response_body,
      status: status_code,
      headers: log_headers,
    };

    self.save_log(&log);

    log
  }
}

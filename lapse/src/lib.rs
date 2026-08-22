use std::path::PathBuf;

pub mod env;
pub mod error;
pub mod log;
pub mod request;
pub mod runner;
pub mod script;
pub mod state;
pub mod tree;

pub use error::{Error, Result};

#[cfg(test)]
mod test;

#[derive(Clone)]
pub struct Lapse {
  path: PathBuf,
}

impl Lapse {
  pub fn open<P: Into<PathBuf>>(path: P) -> crate::Result<Self> {
    let as_buf: PathBuf = path.into();

    let space_dir_path = as_buf.join(".lapse");
    if !space_dir_path.exists() {
      let parent_path = as_buf.parent().ok_or(Error::LapseNotFound)?;
      return Self::open(parent_path);
    }

    Ok(Self { path: as_buf })
  }

  pub fn path(&self) -> &PathBuf {
    &self.path
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
  fn scripts_path(&self) -> PathBuf {
    self.path.join("scripts")
  }
}

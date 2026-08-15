use std::{fs, path::PathBuf};

pub mod env;
pub mod error;
pub mod log;
pub mod lua;
pub mod request;
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
  pub fn init<P: Into<PathBuf>>(path: P) -> crate::Result<Self> {
    let base_path: PathBuf = path.into();

    let space_dirs = ["requests", "env", "scripts", ".lapse"];

    space_dirs
      .into_iter()
      .map(|subdir| {
        let dir_path = base_path.join(subdir);
        fs::create_dir_all(dir_path).map_err(|_| Error::CreateSpaceDir(subdir.to_owned()))
      })
      .collect::<Result<Vec<_>>>()?;

    Ok(Self { path: base_path })
  }
  pub fn open<P: Into<PathBuf>>(path: P) -> crate::Result<Self> {
    let as_buf: PathBuf = path.into();

    let space_dir_path = as_buf.join(".lapse");
    if !space_dir_path.exists() {
      let parent_path = as_buf.parent().ok_or(Error::GetParentDir)?;
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

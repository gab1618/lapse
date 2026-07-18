use std::{
  fs::{self, OpenOptions},
  io::Write,
  path::PathBuf,
};

pub mod env;
pub mod error;
pub mod eval;
pub mod log;
pub mod lua;
pub mod parsing;
pub mod request;
pub mod state;

pub use error::{Error, Result};

#[cfg(test)]
mod test;

pub struct Lapse {
  path: PathBuf,
}

impl Lapse {
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
      .map_err(Error::OpenSampleFile)?;

    let sample_request_contents = include_str!("../assets/request.md");
    f.write_all(sample_request_contents.as_bytes())
      .map_err(Error::WriteSampleFile)?;

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
}

use std::{
  fs::{self, OpenOptions},
  io::Write,
  path::PathBuf,
};

pub mod error;
pub mod request;

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
      .write(true)
      .open(base_path.join("requests").join("request.md"))
      .unwrap();

    let sample_request_contents = include_str!("../assets/request.md");
    f.write_all(sample_request_contents.as_bytes()).unwrap();

    Ok(Self { path: base_path })
  }
  pub fn open<P: Into<PathBuf>>(path: P) -> Self {
    Self { path: path.into() }
  }
  pub fn requests_path(&self) -> PathBuf {
    self.path.join("requests")
  }
}

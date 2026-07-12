use std::ops::Deref;

#[cfg(test)]
use tempfile::TempDir;
use tempfile::tempdir;

use crate::Lapse;

#[cfg(test)]
pub struct TempLapse {
  pub _tempdir: TempDir,
  lapse: Lapse,
}

impl TempLapse {
  pub fn new() -> Self {
    let temp_dir = tempdir().unwrap();
    let lapse = Lapse::init(temp_dir.path()).unwrap();

    Self {
      _tempdir: temp_dir,
      lapse,
    }
  }
}

impl Deref for TempLapse {
  type Target = Lapse;

  fn deref(&self) -> &Self::Target {
    &self.lapse
  }
}

#[test]
fn test_init_space() {
  let temp_dir = tempdir().unwrap();
  Lapse::init(temp_dir.path()).unwrap();

  assert!(temp_dir.path().join("requests").join("request.md").exists());
  assert!(temp_dir.path().join("env").exists());
  assert!(temp_dir.path().join(".lapse").exists());
}

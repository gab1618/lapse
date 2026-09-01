use std::{
  collections::HashMap,
  fs::OpenOptions,
  ops::{Deref, DerefMut},
  path::{Path, PathBuf},
};

use dirs::config_dir;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Config(pub HashMap<String, String>);

impl Deref for Config {
  type Target = HashMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl DerefMut for Config {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Config {
  fn ensure_config_dir() {
    if let Some(config_path) = config_dir() {
      let _ = std::fs::create_dir_all(config_path.join("lapse"));
    }
  }
  fn file_path_from(base: &Path) -> PathBuf {
    base.join("lapse/config.json")
  }
  pub fn read_from(base: &Path) -> Self {
    let f = OpenOptions::new()
      .read(true)
      .open(Self::file_path_from(base))
      .ok();

    f.and_then(|f| serde_json::from_reader(f).ok())
      .unwrap_or_default()
  }
  pub fn save_from(&self, base: &Path) {
    Self::ensure_config_dir();
    let f = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(Self::file_path_from(base))
      .ok();

    f.map(|f| serde_json::to_writer_pretty(f, self));
  }
  pub fn read() -> Self {
    config_dir()
      .map(|f| Self::read_from(&f))
      .unwrap_or_default()
  }
  pub fn save(&self) {
    if let Some(p) = config_dir() {
      self.save_from(&p)
    }
  }
}

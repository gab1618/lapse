use std::fs;
use std::fs::OpenOptions;
use std::ops::Deref;

use lapse_template::templates::LapsePreset;
use tempfile::TempDir;
use tempfile::tempdir;

use crate::Lapse;
use crate::env::Env;
use crate::env::error::EnvError;

pub struct TempLapse {
  pub _tempdir: TempDir,
  lapse: Lapse,
}

impl TempLapse {
  pub fn new() -> Self {
    let temp_dir = tempdir().unwrap();

    let preset = LapsePreset::default();
    preset.load(temp_dir.path()).unwrap();

    let lapse = Lapse::open(temp_dir.path()).unwrap();

    Self {
      _tempdir: temp_dir,
      lapse,
    }
  }

  pub fn set_env(&self, env: &Env, name: &str) -> crate::Result<()> {
    let full_env_path = self.env_path().join(name);
    fs::create_dir_all(&full_env_path).map_err(EnvError::Create)?;

    let variables = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(full_env_path.join("variables.json"))
      .map_err(EnvError::OpenVariables)?;

    serde_json::to_writer(variables, &env.variables).map_err(|_| EnvError::SerializeVariables)?;

    let secrets = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(full_env_path.join("secrets.json"))
      .map_err(EnvError::OpenVariables)?;

    serde_json::to_writer(secrets, &env.secrets).map_err(|_| EnvError::SerializeVariables)?;

    let hooks = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(full_env_path.join("hooks.json"))
      .map_err(EnvError::OpenVariables)?;

    serde_json::to_writer(hooks, &env.hooks).map_err(|_| EnvError::SerializeVariables)?;

    let config = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(full_env_path.join("config.json"))
      .map_err(EnvError::OpenVariables)?;

    serde_json::to_writer(config, &env.hooks).map_err(|_| EnvError::SerializeVariables)?;

    Ok(())
  }
}

impl Deref for TempLapse {
  type Target = Lapse;

  fn deref(&self) -> &Self::Target {
    &self.lapse
  }
}

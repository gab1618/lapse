use std::{
  collections::HashMap,
  fs::{self, OpenOptions},
  io::{BufReader, Read, Write},
};

use crate::{Lapse, env::error::EnvError};

#[cfg(test)]
mod test;

pub mod error;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Env {
  pub variables: HashMap<String, EnvVariable>,
}

#[derive(PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum EnvVariable {
  String(String),
  Number(f64),
}

impl From<String> for EnvVariable {
  fn from(value: String) -> Self {
    Self::String(value)
  }
}

impl From<&str> for EnvVariable {
  fn from(value: &str) -> Self {
    Self::String(value.to_string())
  }
}
impl From<f64> for EnvVariable {
  fn from(value: f64) -> Self {
    Self::Number(value)
  }
}

impl Lapse {
  pub fn switch_env(&self, name: &str) -> crate::Result<()> {
    let state_path = self.state_path();
    fs::create_dir_all(&state_path).map_err(EnvError::EnsureStateDir)?;
    let env_state_file = state_path.join("env");

    let env_file_path = self.env_path().join(name);
    if !env_file_path.exists() {
      return Err(EnvError::NonExistentEnv(name.to_string()).into());
    }

    let mut f = OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open(env_state_file)
      .map_err(EnvError::OpenStateFile)?;

    f.write_all(name.as_bytes()).map_err(EnvError::SaveState)?;

    Ok(())
  }

  // TODO: not being in a env is a thing
  pub fn current_env(&self) -> crate::Result<String> {
    let state_path = self.state_path();
    fs::create_dir_all(&state_path).map_err(EnvError::EnsureStateDir)?;
    let env_state_file = state_path.join("env");

    let f = OpenOptions::new()
      .read(true)
      .open(env_state_file)
      .map_err(EnvError::OpenStateFile)?;
    let mut r = BufReader::new(f);

    let mut curr_env = String::new();
    r.read_to_string(&mut curr_env)
      .map_err(EnvError::ReadState)?;

    Ok(curr_env)
  }
  pub fn get_env(&self, name: &str) -> Env {
    let full_env_path = self.env_path().join(name);

    let f = OpenOptions::new().read(true).open(full_env_path).unwrap();

    let parsed: Env = serde_json::from_reader(f).unwrap();
    parsed
  }
  pub fn set_env(&self, env: &Env, name: &str) {
    let full_env_path = self.env_path().join(name);

    let f = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(full_env_path)
      .unwrap();

    serde_json::to_writer(f, env).unwrap();
  }
}

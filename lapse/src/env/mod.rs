use std::{collections::HashMap, fs::OpenOptions};

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
    let env_file_path = self.env_path().join(name).with_extension("json");
    if !env_file_path.exists() {
      return Err(EnvError::NonExistentEnv(name.to_string()).into());
    }

    self.set_state("env", name)?;

    Ok(())
  }

  pub fn current_env(&self) -> crate::Result<Option<String>> {
    self.get_state("env")
  }
  pub fn get_env(&self, name: &str) -> Env {
    let full_env_path = self.env_path().join(name).with_extension("json");

    let f = OpenOptions::new().read(true).open(full_env_path).unwrap();

    let parsed: Env = serde_json::from_reader(f).unwrap();
    parsed
  }
  pub fn set_env(&self, env: &Env, name: &str) {
    let full_env_path = self.env_path().join(name).with_extension("json");

    let f = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(full_env_path)
      .unwrap();

    serde_json::to_writer(f, env).unwrap();
  }
}

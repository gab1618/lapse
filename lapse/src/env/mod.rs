use std::{collections::HashMap, fs::OpenOptions};

use crate::{Lapse, env::error::EnvError};

#[cfg(test)]
mod test;

pub mod error;

pub type Env = HashMap<String, EnvVariable>;

#[derive(PartialEq, Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum EnvVariable {
  Null,
  Boolean(bool),
  Integer(i64),
  Number(f64),
  String(String),
  Object(HashMap<String, EnvVariable>),
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
  pub fn get_env(&self, name: &str) -> crate::Result<Env> {
    let full_env_path = self.env_path().join(name).with_extension("json");

    let f = OpenOptions::new()
      .read(true)
      .open(full_env_path)
      .map_err(EnvError::OpenEnvFile)?;

    let parsed: Env = serde_json::from_reader(f).map_err(|_| EnvError::ParseEnv)?;

    Ok(parsed)
  }
  pub fn set_env(&self, env: &Env, name: &str) -> crate::Result<()> {
    let full_env_path = self.env_path().join(name).with_extension("json");

    let f = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(full_env_path)
      .map_err(EnvError::OpenEnvFile)?;

    serde_json::to_writer(f, env).map_err(|_| EnvError::SerializeEnv)?;

    Ok(())
  }
}

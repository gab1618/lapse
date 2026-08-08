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

impl From<bool> for EnvVariable {
  fn from(value: bool) -> Self {
    Self::Boolean(value)
  }
}
impl From<f64> for EnvVariable {
  fn from(value: f64) -> Self {
    Self::Number(value)
  }
}
impl From<i64> for EnvVariable {
  fn from(value: i64) -> Self {
    Self::Integer(value)
  }
}
impl From<String> for EnvVariable {
  fn from(value: String) -> Self {
    Self::String(value)
  }
}
impl From<HashMap<String, EnvVariable>> for EnvVariable {
  fn from(value: HashMap<String, EnvVariable>) -> Self {
    Self::Object(value)
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

  pub fn current_env(&self) -> crate::Result<String> {
    let env_name = self.get_state("env")?.unwrap_or("default".to_string());
    Ok(env_name)
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

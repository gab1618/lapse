use std::{collections::HashMap, fs::OpenOptions};

use crate::{Lapse, env::error::EnvError};

#[cfg(test)]
mod test;

pub mod error;

#[derive(Default)]
pub struct Env {
  pub variables: HashMap<String, EnvValue>,
}

#[derive(PartialEq, Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum EnvValue {
  Null,
  Boolean(bool),
  Integer(i64),
  Number(f64),
  String(String),
  Object(HashMap<String, EnvValue>),
}

impl From<bool> for EnvValue {
  fn from(value: bool) -> Self {
    Self::Boolean(value)
  }
}
impl From<f64> for EnvValue {
  fn from(value: f64) -> Self {
    Self::Number(value)
  }
}
impl From<i64> for EnvValue {
  fn from(value: i64) -> Self {
    Self::Integer(value)
  }
}
impl From<String> for EnvValue {
  fn from(value: String) -> Self {
    Self::String(value)
  }
}
impl From<HashMap<String, EnvValue>> for EnvValue {
  fn from(value: HashMap<String, EnvValue>) -> Self {
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

    let parsed_variables: HashMap<String, EnvValue> =
      serde_json::from_reader(f).map_err(|_| EnvError::ParseEnv)?;

    Ok(Env {
      variables: parsed_variables,
    })
  }
  pub fn set_env(&self, env: &Env, name: &str) -> crate::Result<()> {
    let full_env_path = self.env_path().join(name).with_extension("json");

    let f = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(full_env_path)
      .map_err(EnvError::OpenEnvFile)?;

    serde_json::to_writer(f, &env.variables).map_err(|_| EnvError::SerializeEnv)?;

    Ok(())
  }
}

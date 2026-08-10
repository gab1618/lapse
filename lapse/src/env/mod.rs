use std::{
  collections::HashMap,
  fs::{self, OpenOptions},
};

use crate::{
  Lapse,
  env::{
    error::EnvError,
    hook::{Event, HookEntry},
  },
};

#[cfg(test)]
mod test;

pub mod error;
pub mod hook;

#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Default)]
pub struct Env {
  pub variables: HashMap<String, EnvValue>,
  pub secrets: HashMap<String, EnvValue>,
  pub hooks: HashMap<Event, HookEntry>,
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
    let env_path = self.env_path().join(name);
    if !env_path.exists() {
      return Err(EnvError::NonExistentEnv(name.to_string()).into());
    }

    self.set_state("env", name)?;

    Ok(())
  }

  /// Every file is optional. If it doesn't exist, we just treat it as if it was empty. Therefore,
  /// it is impossible to face some error when dealing with these files
  fn read_env_resource(&self, env: &str, name: &str) -> HashMap<String, EnvValue> {
    let full_env_path = self.env_path().join(env);

    let f = OpenOptions::new()
      .read(true)
      .open(full_env_path.join(name))
      .map_err(EnvError::OpenVariables);

    f.map(|inner| serde_json::from_reader(inner).ok())
      .ok()
      .flatten()
      .unwrap_or_default()
  }

  pub fn current_env(&self) -> crate::Result<String> {
    let env_name = self.get_state("env")?.unwrap_or("default".to_string());
    Ok(env_name)
  }
  pub fn get_env(&self, name: &str) -> crate::Result<Env> {
    let full_env_path = self.env_path().join(name);

    let variables = self.read_env_resource(name, "variables.json");
    let secrets = self.read_env_resource(name, "secrets.json");

    let hooks_f = OpenOptions::new()
      .read(true)
      .open(full_env_path.join("hooks.json"))
      .ok();

    let hooks = hooks_f
      .and_then(|f| serde_json::from_reader::<_, HashMap<Event, HookEntry>>(f).ok())
      .unwrap_or_default();

    Ok(Env {
      variables,
      secrets,
      hooks,
    })
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

    Ok(())
  }
}

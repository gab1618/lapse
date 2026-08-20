use std::{collections::HashMap, fs::OpenOptions};

use crate::{
  Lapse,
  env::{
    config::EnvConfig,
    error::EnvError,
    hook::{Event, HookEntry},
  },
  runner::value::Value,
};

#[cfg(test)]
mod test;

pub mod config;
pub mod error;
pub mod hook;

#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Default)]
pub struct Env {
  pub variables: HashMap<String, Value>,
  pub secrets: HashMap<String, Value>,
  pub hooks: HashMap<Event, HookEntry>,
  pub config: EnvConfig,
}

impl std::ops::Add for Env {
  type Output = Env;

  /// Adds two envs together, prioritizing the right operand
  fn add(mut self, rhs: Self) -> Self::Output {
    for (key, variable) in rhs.variables {
      self.variables.insert(key, variable);
    }

    for (key, secret) in rhs.secrets {
      self.secrets.insert(key, secret);
    }

    for (key, hook) in rhs.hooks {
      self.hooks.insert(key, hook);
    }

    self
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
  fn read_env_resource(&self, env: &str, name: &str) -> HashMap<String, Value> {
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

  pub fn current_env(&self) -> String {
    self
      .get_state("env")
      .ok()
      .flatten()
      .unwrap_or("default".to_string())
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

    let config_f = OpenOptions::new()
      .read(true)
      .open(full_env_path.join("config.json"))
      .ok();
    let config: EnvConfig = config_f
      .and_then(|f| serde_json::from_reader(f).ok())
      .unwrap_or_default();

    let mut resulting_env = Env {
      variables,
      secrets,
      hooks,
      config,
    };

    let mut env_name_segments = name.split('/').collect::<Vec<&str>>();
    if env_name_segments.len() > 1 {
      env_name_segments.pop();
      let parent_name = env_name_segments.join("/");
      let parent = self.get_env(&parent_name)?;
      resulting_env = parent + resulting_env;
    }

    Ok(resulting_env)
  }
}

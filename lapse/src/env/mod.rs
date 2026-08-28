use std::{collections::HashMap, fs::OpenOptions, path::Path};

use serde::de::DeserializeOwned;

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

    self.config = rhs.config;

    self
  }
}

impl Env {
  fn read_resource<T: DeserializeOwned + Default, P: AsRef<Path>>(full_path: P) -> T {
    let f = OpenOptions::new().read(true).open(full_path);

    f.map(|f| serde_json::from_reader(f).ok())
      .ok()
      .flatten()
      .unwrap_or_default()
  }

  /// Every file is optional. If it doesn't exist, we just treat it as if it was empty. Therefore,
  /// it is impossible to face some error when dealing with these files
  pub fn read<P: AsRef<Path>>(path: P) -> Self {
    let path = path.as_ref();

    Self {
      variables: Self::read_resource(path.join("variables.json")),
      secrets: Self::read_resource(path.join("secrets.json")),
      hooks: Self::read_resource(path.join("hooks.json")),
      config: Self::read_resource(path.join("config.json")),
    }
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

  pub fn current_env(&self) -> Option<String> {
    self.get_state("env").ok().flatten()
  }
  pub fn get_env(&self, name: &str) -> crate::Result<Env> {
    let full_env_path = self.env_path().join(name);

    let mut resulting_env = Env::read(full_env_path);

    let mut env_name_segments = name.split('/').collect::<Vec<&str>>();

    // Env is not root, therefore we read the parent as well
    if env_name_segments.len() > 1 {
      env_name_segments.pop();
      let parent_name = env_name_segments.join("/");
      let parent = self.get_env(&parent_name)?;
      resulting_env = parent + resulting_env;
    } else {
      // Env is root, so we read the base env as a parent
      let parent = Env::read(self.env_path());
      resulting_env = parent + resulting_env;
    }

    Ok(resulting_env)
  }
}

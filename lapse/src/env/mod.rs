use std::{
  collections::HashMap,
  fs::{self, OpenOptions},
  io::{BufReader, Read, Write},
};

use crate::Lapse;

#[cfg(test)]
mod test;

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
  // TODO: later, check the existence of the env
  pub fn switch_env(&self, name: &str) {
    let state_path = self.state_path();
    fs::create_dir_all(&state_path).unwrap();
    let env_state_file = state_path.join("env");

    let mut f = OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open(env_state_file)
      .unwrap();

    f.write_all(name.as_bytes()).unwrap();
  }

  // TODO: not being in a env is a thing
  pub fn current_env(&self) -> String {
    let state_path = self.state_path();
    fs::create_dir_all(&state_path).unwrap();
    let env_state_file = state_path.join("env");

    let f = OpenOptions::new().read(true).open(env_state_file).unwrap();
    let mut r = BufReader::new(f);

    let mut curr_env = String::new();
    r.read_to_string(&mut curr_env).unwrap();

    curr_env
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

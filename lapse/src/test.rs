use std::fs;
use std::fs::OpenOptions;
use std::ops::Deref;

use tempfile::TempDir;
use tempfile::tempdir;

use crate::env::Env;
use crate::env::error::EnvError;
use crate::request::runner::RequestRunner;
use crate::{Lapse, env::EnvValue};

pub struct TempLapse {
  pub _tempdir: TempDir,
  lapse: Lapse,
}

impl TempLapse {
  pub fn new() -> Self {
    let temp_dir = tempdir().unwrap();
    let lapse = Lapse::init(temp_dir.path()).unwrap();

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

    Ok(())
  }
}

impl Deref for TempLapse {
  type Target = Lapse;

  fn deref(&self) -> &Self::Target {
    &self.lapse
  }
}

#[test]
fn test_init_space() {
  let temp_dir = tempdir().unwrap();
  Lapse::init(temp_dir.path()).unwrap();
}

#[test]
fn test_get_eval_ctx_loads_current_env_variables() {
  let lapse = TempLapse::new();

  let mut env = Env::default();
  env
    .variables
    .insert("name".to_string(), EnvValue::String("Jane".to_string()));
  env
    .variables
    .insert("age".to_string(), EnvValue::Number(30.0));

  lapse.set_env(&env, "prod").unwrap();
  lapse.switch_env("prod").unwrap();

  let ctx = RequestRunner::new(lapse.get_runtime().unwrap(), Default::default());

  assert_eq!(ctx.eval("${env.name} is ${env.age}").unwrap(), "Jane is 30");
}

#[test]
fn test_get_eval_ctx_defaults_to_empty_without_current_env() {
  let lapse = TempLapse::new();

  let ctx = RequestRunner::new(lapse.get_runtime().unwrap(), Default::default());

  assert_eq!(ctx.eval("${env.missing}").unwrap(), "null");
}

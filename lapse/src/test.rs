use std::ops::Deref;

#[cfg(test)]
use tempfile::TempDir;
use tempfile::tempdir;

use crate::{
  Lapse,
  env::{Env, EnvVariable},
  parsing::RequestTokenizer,
};

#[cfg(test)]
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

  assert!(temp_dir.path().join("requests").join("request.md").exists());
  assert!(temp_dir.path().join("env").exists());
  assert!(temp_dir.path().join(".lapse").exists());
}

#[test]
fn test_get_eval_ctx_loads_current_env_variables() {
  let lapse = TempLapse::new();

  let mut variables = std::collections::HashMap::new();
  variables.insert("name".to_string(), EnvVariable::String("Jane".to_string()));
  variables.insert("age".to_string(), EnvVariable::Number(30.0));

  lapse.set_env(&Env { variables }, "prod").unwrap();
  lapse.switch_env("prod").unwrap();

  let ctx = lapse.get_eval_ctx().unwrap();
  let tokens = RequestTokenizer::new("${env.name} is ${env.age}").tokenize();

  assert_eq!(ctx.eval(tokens).unwrap(), "Jane is 30");
}

#[test]
fn test_get_eval_ctx_defaults_to_empty_without_current_env() {
  let lapse = TempLapse::new();

  let ctx = lapse.get_eval_ctx().unwrap();
  let tokens = RequestTokenizer::new("${env.missing}").tokenize();

  assert_eq!(ctx.eval(tokens).unwrap(), "null");
}

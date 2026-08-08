use std::ops::Deref;

use tempfile::TempDir;
use tempfile::tempdir;

use crate::{Lapse, env::EnvVariable};

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
}

#[test]
fn test_get_eval_ctx_loads_current_env_variables() {
  let lapse = TempLapse::new();

  let mut env = std::collections::HashMap::new();
  env.insert("name".to_string(), EnvVariable::String("Jane".to_string()));
  env.insert("age".to_string(), EnvVariable::Number(30.0));

  lapse.set_env(&env, "prod").unwrap();
  lapse.switch_env("prod").unwrap();

  let ctx = lapse.get_eval_ctx().unwrap();

  assert_eq!(ctx.eval("${env.name} is ${env.age}").unwrap(), "Jane is 30");
}

#[test]
fn test_get_eval_ctx_defaults_to_empty_without_current_env() {
  let lapse = TempLapse::new();

  let ctx = lapse.get_eval_ctx().unwrap();

  assert_eq!(ctx.eval("${env.missing}").unwrap(), "null");
}

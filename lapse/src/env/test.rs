use std::collections::HashMap;

use tempfile::tempdir;

use crate::{
  Lapse,
  env::{Env, EnvVariable},
};

#[test]
fn test_switch_env() {
  let temp_dir = tempdir().unwrap();
  let lapse = Lapse::init(temp_dir.path()).unwrap();

  lapse.switch_env("prod");

  assert_eq!(lapse.current_env(), "prod");
}

#[test]
fn test_read_env() {
  let temp_dir = tempdir().unwrap();
  let lapse = Lapse::init(temp_dir.path()).unwrap();

  let mut ex_variables = HashMap::new();

  ex_variables.insert("name".to_string(), "John".into());

  let ex_env = Env {
    variables: ex_variables,
  };

  lapse.set_env(&ex_env, "env");

  let found_env = lapse.get_env("env");

  let found_name = found_env.variables.get("name").unwrap();
  assert_eq!(found_name, &EnvVariable::String("John".into()));
}

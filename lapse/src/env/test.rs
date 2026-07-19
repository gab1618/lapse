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

  let ex_env = Env {
    variables: HashMap::default(),
  };

  lapse.set_env(&ex_env, "prod").unwrap();

  lapse.switch_env("prod").unwrap();

  assert_eq!(lapse.current_env().unwrap().unwrap(), "prod");

  lapse.switch_env("dev").unwrap_err();
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

  lapse.set_env(&ex_env, "env").unwrap();

  let found_env = lapse.get_env("env").unwrap();

  let found_name = found_env.variables.get("name").unwrap();
  assert_eq!(found_name, &EnvVariable::String("John".into()));
}

#[test]
fn test_parse_asset_env_json() {
  let content =
    std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/env.json")).unwrap();

  let parsed: Env = serde_json::from_str(&content).unwrap();

  assert_eq!(
    parsed.variables.get("name").unwrap(),
    &EnvVariable::String("John".into())
  );
}

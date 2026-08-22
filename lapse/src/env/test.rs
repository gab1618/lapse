use std::collections::HashMap;

use crate::{
  env::{
    Env,
    hook::{Event, HookEntry},
  },
  runner::value::Value,
  test::TempLapse,
};

#[test]
fn test_switch_env() {
  let lapse = TempLapse::new();

  let ex_env = Env::default();

  lapse.set_env(&ex_env, "prod").unwrap();

  lapse.switch_env("prod").unwrap();

  assert_eq!(lapse.current_env(), Some("prod".to_string()));

  lapse.switch_env("dev").unwrap_err();
}

#[test]
fn test_read_env() {
  let lapse = TempLapse::new();

  let mut ex_env = Env::default();

  ex_env
    .variables
    .insert("name".to_string(), Value::String("John".to_string()));

  let base_env = lapse.get_env("").unwrap();

  ex_env.hooks.insert(
    Event::PreRequest,
    HookEntry {
      enabled: true,
      scripts: vec![],
    },
  );

  lapse.set_env(&ex_env, "env").unwrap();

  let found_env = lapse.get_env("env").unwrap();

  assert_eq!(base_env + ex_env, found_env);
}

#[test]
fn test_parse_asset_env_json() {
  let content =
    std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/env.json")).unwrap();

  let parsed: HashMap<String, Value> = serde_json::from_str(&content).unwrap();

  assert_eq!(parsed.get("name").unwrap(), &Value::String("John".into()));
}

#[test]
fn test_env_inheritance() {
  let lapse = TempLapse::new();

  let mut child = Env::default();
  let mut parent = Env::default();

  child.variables.insert("auth".into(), true.into());
  parent.variables.insert("auth".into(), false.into());
  parent.variables.insert("name".into(), "John Doe".into());

  lapse.set_env(&parent, "default").unwrap();
  lapse.set_env(&child, "default/auth").unwrap();

  let child = lapse.get_env("default/auth").unwrap();

  let found_auth = child.variables.get("auth").unwrap();
  assert_eq!(found_auth, &true.into());

  let found_name = child.variables.get("name").unwrap();
  assert_eq!(found_name, &Value::String("John Doe".into()));
}

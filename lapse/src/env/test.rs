use std::collections::HashMap;

use crate::{
  env::{
    Env, EnvValue,
    hook::{Event, HookEntry},
  },
  test::TempLapse,
};

#[test]
fn test_switch_env() {
  let lapse = TempLapse::new();

  let ex_env = Env::default();

  lapse.set_env(&ex_env, "prod").unwrap();

  lapse.switch_env("prod").unwrap();

  assert_eq!(lapse.current_env(), "prod");

  lapse.switch_env("dev").unwrap_err();
}

#[test]
fn test_read_env() {
  let lapse = TempLapse::new();

  let mut ex_env = Env::default();

  ex_env
    .variables
    .insert("name".to_string(), EnvValue::String("John".to_string()));

  ex_env.hooks.insert(
    Event::PreRequest,
    HookEntry {
      enabled: true,
      scripts: vec![],
    },
  );

  lapse.set_env(&ex_env, "env").unwrap();

  let found_env = lapse.get_env("env").unwrap();

  assert_eq!(ex_env, found_env);
}

#[test]
fn test_parse_asset_env_json() {
  let content =
    std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/env.json")).unwrap();

  let parsed: HashMap<String, EnvValue> = serde_json::from_str(&content).unwrap();

  assert_eq!(
    parsed.get("name").unwrap(),
    &EnvValue::String("John".into())
  );
}

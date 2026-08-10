use mlua::Lua;

use crate::{env::EnvValue, script::Runtime};
use std::collections::HashMap;

fn eval_ctx(variables: HashMap<String, EnvValue>) -> crate::Result<Runtime> {
  let runtime = Lua::new();

  runtime.globals().set("env", variables)?;

  Ok(Runtime(runtime))
}

#[test]
fn test_evaluates_plain_string() {
  let ctx = eval_ctx(Default::default()).unwrap();

  assert_eq!(
    ctx.eval("just a plain string").unwrap(),
    "just a plain string"
  );
}

#[test]
fn test_evaluates_var_expression() {
  let ctx = eval_ctx(HashMap::from([(
    "name".to_string(),
    EnvValue::String("John".to_string()),
  )]))
  .unwrap();

  assert_eq!(
    ctx.eval("{\n  \"name\": ${env.name}\n}").unwrap(),
    "{\n  \"name\": John\n}"
  );
}

#[test]
fn test_var_missing_returns_null() {
  let ctx = eval_ctx(Default::default()).unwrap();

  assert_eq!(ctx.eval("${env.missing}").unwrap(), "null");
}

#[test]
fn test_evaluates_number_var_expression() {
  let ctx = eval_ctx(HashMap::from([("age".to_string(), 42.into())])).unwrap();

  assert_eq!(ctx.eval("${env.age}").unwrap(), "42");
}

#[test]
fn test_evaluates_object_var_expression() {
  let object = EnvValue::Object(HashMap::from([(
    "city".to_string(),
    EnvValue::String("NYC".to_string()),
  )]));
  let ctx = eval_ctx(HashMap::from([("address".to_string(), object)])).unwrap();

  assert_eq!(ctx.eval("${env.address}").unwrap(), "{\"city\":NYC}");
}

#[test]
fn test_evaluates_arithmetic_expression() {
  let ctx = eval_ctx(Default::default()).unwrap();

  assert_eq!(ctx.eval("${1 + 2}").unwrap(), "3");
}

#[test]
fn test_evaluates_boolean_expression() {
  let ctx = eval_ctx(Default::default()).unwrap();

  assert_eq!(ctx.eval("${1 == 1}").unwrap(), "true");
}

#[test]
fn test_evaluates_table_expression() {
  let ctx = eval_ctx(Default::default()).unwrap();

  assert_eq!(
    ctx.eval("${ {a = 1, b = \"x\"} }").unwrap(),
    "{\"a\":1,\"b\":x}"
  );
}

#[test]
fn test_env_variable_display() {
  assert_eq!(EnvValue::Null.to_string(), "null");
  assert_eq!(EnvValue::Boolean(true).to_string(), "true");
  assert_eq!(EnvValue::Integer(42).to_string(), "42");
  assert_eq!(EnvValue::String("hi".to_owned()).to_string(), "hi");
}

use std::collections::HashMap;

use mlua::Lua;

use crate::{
  env::Env,
  runner::{Runner, value::Value},
  test::TempLapse,
};

#[test]
fn test_get_eval_ctx_loads_current_env_variables() {
  let lapse = TempLapse::new();

  let mut env = Env::default();
  env
    .variables
    .insert("name".to_string(), Value::String("Jane".to_string()));
  env.variables.insert("age".to_string(), Value::Number(30.0));

  lapse.set_env(&env, "prod").unwrap();
  lapse.switch_env("prod").unwrap();

  let ctx = Runner::from_space(&lapse).unwrap();

  assert_eq!(ctx.eval("${Env.name} is ${Env.age}").unwrap(), "Jane is 30");
}

#[test]
fn test_get_eval_ctx_defaults_to_empty_without_current_env() {
  let lapse = TempLapse::new();

  let ctx = Runner::from_space(&lapse).unwrap();

  assert_eq!(ctx.eval("${Env.missing}").unwrap(), "null");
}

fn eval_ctx(variables: HashMap<String, Value>) -> crate::Result<Runner> {
  let runtime = Lua::new();

  runtime.globals().set("Env", variables)?;

  Ok(Runner::new(runtime, Default::default(), Default::default()))
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
    Value::String("John".to_string()),
  )]))
  .unwrap();

  assert_eq!(
    ctx.eval("{\n  \"name\": ${Env.name}\n}").unwrap(),
    "{\n  \"name\": John\n}"
  );
}

#[test]
fn test_var_missing_returns_null() {
  let ctx = eval_ctx(Default::default()).unwrap();

  assert_eq!(ctx.eval("${Env.missing}").unwrap(), "null");
}

#[test]
fn test_evaluates_number_var_expression() {
  let ctx = eval_ctx(HashMap::from([("age".to_string(), 42.into())])).unwrap();

  assert_eq!(ctx.eval("${Env.age}").unwrap(), "42");
}

#[test]
fn test_evaluates_object_var_expression() {
  let object = Value::Object(HashMap::from([(
    "city".to_string(),
    Value::String("NYC".to_string()),
  )]));
  let ctx = eval_ctx(HashMap::from([("address".to_string(), object)])).unwrap();

  assert_eq!(ctx.eval("${Env.address}").unwrap(), "{\"city\":NYC}");
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
  assert_eq!(Value::Null.to_string(), "null");
  assert_eq!(Value::Boolean(true).to_string(), "true");
  assert_eq!(Value::Integer(42).to_string(), "42");
  assert_eq!(Value::String("hi".to_owned()).to_string(), "hi");
}

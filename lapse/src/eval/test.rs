use super::EvalCtx;
use crate::{env::EnvVariable, test::TempLapse};
use std::{collections::HashMap, fs};

fn eval_ctx(variables: &[(&str, &str)]) -> crate::Result<EvalCtx> {
  let variables = variables
    .iter()
    .map(|(k, v)| (k.to_string(), EnvVariable::String(v.to_string())))
    .collect::<HashMap<_, _>>();

  EvalCtx::new(variables, Default::default())
}

#[test]
fn test_evaluates_plain_string() {
  let ctx = eval_ctx(&[]).unwrap();

  assert_eq!(
    ctx.eval("just a plain string").unwrap(),
    "just a plain string"
  );
}

#[test]
fn test_evaluates_var_expression() {
  let ctx = eval_ctx(&[("name", "John")]).unwrap();

  assert_eq!(
    ctx.eval("{\n  \"name\": ${env.name}\n}").unwrap(),
    "{\n  \"name\": John\n}"
  );
}

#[test]
fn test_var_missing_returns_null() {
  let ctx = eval_ctx(&[]).unwrap();

  assert_eq!(ctx.eval("${env.missing}").unwrap(), "null");
}

#[test]
fn test_evaluates_number_var_expression() {
  let ctx = EvalCtx::new(
    HashMap::from([("age".to_string(), EnvVariable::Number(42.0))]),
    Default::default(),
  )
  .unwrap();

  assert_eq!(ctx.eval("${env.age}").unwrap(), "42");
}

#[test]
fn test_evaluates_object_var_expression() {
  let object = EnvVariable::Object(HashMap::from([(
    "city".to_string(),
    EnvVariable::String("NYC".to_string()),
  )]));
  let ctx = EvalCtx::new(
    HashMap::from([("address".to_string(), object)]),
    Default::default(),
  )
  .unwrap();

  assert_eq!(ctx.eval("${env.address}").unwrap(), "{\"city\":NYC}");
}

#[test]
fn test_evaluates_arithmetic_expression() {
  let ctx = eval_ctx(&[]).unwrap();

  assert_eq!(ctx.eval("${1 + 2}").unwrap(), "3");
}

#[test]
fn test_evaluates_boolean_expression() {
  let ctx = eval_ctx(&[]).unwrap();

  assert_eq!(ctx.eval("${1 == 1}").unwrap(), "true");
}

#[test]
fn test_evaluates_table_expression() {
  let ctx = eval_ctx(&[]).unwrap();

  assert_eq!(
    ctx.eval("${ {a = 1, b = \"x\"} }").unwrap(),
    "{\"a\":1,\"b\":x}"
  );
}

#[test]
fn test_env_variable_display() {
  assert_eq!(EnvVariable::Null.to_string(), "null");
  assert_eq!(EnvVariable::Boolean(true).to_string(), "true");
  assert_eq!(EnvVariable::Integer(42).to_string(), "42");
  assert_eq!(EnvVariable::String("hi".to_owned()).to_string(), "hi");
}

#[test]
fn test_load_secret() {
  let lapse = TempLapse::new();
  let ex_secret_content = include_str!("../../assets/secrets.json");
  fs::write(lapse.secrets_path(), ex_secret_content).unwrap();
  let ctx = lapse.get_eval_ctx().unwrap();

  let result = ctx.eval("${secret.password}").unwrap();
  assert_eq!(result, "sshhhh");
}

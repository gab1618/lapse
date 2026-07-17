pub mod request;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{Lapse, env::EnvVariable, parsing::RequestToken};
use mlua::{IntoLua, Lua, Value};

pub struct EvalCtx {
  variables: Rc<RefCell<HashMap<String, EnvVariable>>>,
  runtime: Lua,
}

impl EvalCtx {
  pub fn new(variables: HashMap<String, EnvVariable>) -> Self {
    let variables = Rc::new(RefCell::new(variables));
    let runtime = Lua::new();

    let lookup = Rc::clone(&variables);

    let var_fn = runtime
      .create_function(move |lua, name: String| match lookup.borrow().get(&name) {
        Some(value) => value.clone().into_lua(lua),
        None => Ok(Value::Nil),
      })
      .unwrap();

    runtime.globals().set("var", var_fn).unwrap();

    Self { variables, runtime }
  }

  pub fn set_variable(&self, name: impl Into<String>, value: impl Into<EnvVariable>) {
    self
      .variables
      .borrow_mut()
      .insert(name.into(), value.into());
  }

  pub fn eval(&self, doc: Vec<RequestToken>) -> String {
    let mut result = String::new();

    for token in doc {
      match token {
        RequestToken::String(inner) => {
          result.push_str(&inner);
        }
        RequestToken::Expr(inner) => {
          let value: EnvVariable = self.runtime.load(inner).eval().unwrap();
          result.push_str(&value.to_string());
        }
      }
    }

    result
  }
}

impl Lapse {
  pub fn get_eval_ctx(&self) -> EvalCtx {
    let variables = self
      .current_env()
      .ok()
      .flatten()
      .map(|name| self.get_env(&name).variables)
      .unwrap_or_default();

    EvalCtx::new(variables)
  }
}

#[cfg(test)]
mod test {
  use super::EvalCtx;
  use crate::{env::EnvVariable, parsing::RequestTokenizer};
  use std::collections::HashMap;

  fn eval_ctx(variables: &[(&str, &str)]) -> EvalCtx {
    let variables = variables
      .iter()
      .map(|(k, v)| (k.to_string(), EnvVariable::String(v.to_string())))
      .collect::<HashMap<_, _>>();

    EvalCtx::new(variables)
  }

  fn eval(ctx: &EvalCtx, src: &str) -> String {
    let tokens = RequestTokenizer::new(src).tokenize();
    ctx.eval(tokens)
  }

  #[test]
  fn test_evaluates_plain_string() {
    let ctx = eval_ctx(&[]);

    assert_eq!(eval(&ctx, "just a plain string"), "just a plain string");
  }

  #[test]
  fn test_evaluates_var_expression() {
    let ctx = eval_ctx(&[("name", "John")]);

    assert_eq!(
      eval(&ctx, "{\n  \"name\": ${var(\"name\")}\n}"),
      "{\n  \"name\": John\n}"
    );
  }

  #[test]
  fn test_var_missing_returns_null() {
    let ctx = eval_ctx(&[]);

    assert_eq!(eval(&ctx, "${var(\"missing\")}"), "null");
  }

  #[test]
  fn test_set_variable_updates_lookup() {
    let ctx = eval_ctx(&[]);
    ctx.set_variable("name", "Jane");

    assert_eq!(eval(&ctx, "${var(\"name\")}"), "Jane");
  }

  #[test]
  fn test_evaluates_number_var_expression() {
    let ctx = EvalCtx::new(HashMap::from([(
      "age".to_string(),
      EnvVariable::Number(42.0),
    )]));

    assert_eq!(eval(&ctx, "${var(\"age\")}"), "42");
  }

  #[test]
  fn test_evaluates_object_var_expression() {
    let object = EnvVariable::Object(HashMap::from([(
      "city".to_string(),
      EnvVariable::String("NYC".to_string()),
    )]));
    let ctx = EvalCtx::new(HashMap::from([("address".to_string(), object)]));

    assert_eq!(eval(&ctx, "${var(\"address\")}"), "{\"city\":NYC}");
  }

  #[test]
  fn test_evaluates_arithmetic_expression() {
    let ctx = eval_ctx(&[]);

    assert_eq!(eval(&ctx, "${1 + 2}"), "3");
  }

  #[test]
  fn test_evaluates_boolean_expression() {
    let ctx = eval_ctx(&[]);

    assert_eq!(eval(&ctx, "${1 == 1}"), "true");
  }

  #[test]
  fn test_evaluates_table_expression() {
    let ctx = eval_ctx(&[]);

    assert_eq!(eval(&ctx, "${ {a = 1, b = \"x\"} }"), "{\"a\":1,\"b\":x}");
  }

  #[test]
  fn test_env_variable_display() {
    assert_eq!(EnvVariable::Null.to_string(), "null");
    assert_eq!(EnvVariable::Boolean(true).to_string(), "true");
    assert_eq!(EnvVariable::Integer(42).to_string(), "42");
    assert_eq!(EnvVariable::String("hi".to_owned()).to_string(), "hi");
  }
}

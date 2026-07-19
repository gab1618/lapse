pub mod request;

use std::collections::HashMap;

use crate::{Lapse, env::EnvVariable, parsing::RequestToken};
use mlua::Lua;

pub struct EvalCtx {
  runtime: Lua,
}

impl EvalCtx {
  pub fn new(variables: HashMap<String, EnvVariable>) -> crate::Result<Self> {
    let runtime = Lua::new();

    let env_table = runtime.create_table()?;

    for (key, value) in variables.into_iter() {
      env_table.set(key, value)?;
    }

    runtime.globals().set("env", env_table)?;

    Ok(Self { runtime })
  }

  pub fn eval(&self, doc: Vec<RequestToken>) -> crate::Result<String> {
    let mut result = String::new();

    for token in doc {
      match token {
        RequestToken::String(inner) => {
          result.push_str(&inner);
        }
        RequestToken::Expr(inner) => {
          let value: EnvVariable = self.runtime.load(inner).eval()?;
          result.push_str(&value.to_string());
        }
      }
    }

    Ok(result)
  }
}

impl Lapse {
  pub fn get_eval_ctx(&self) -> crate::Result<EvalCtx> {
    let variables = self
      .current_env()
      .ok()
      .flatten()
      .map(|name| self.get_env(&name).unwrap_or_default().variables)
      .unwrap_or_default();

    EvalCtx::new(variables)
  }
}

#[cfg(test)]
mod test {
  use super::EvalCtx;
  use crate::{env::EnvVariable, parsing::RequestTokenizer};
  use std::collections::HashMap;

  fn eval_ctx(variables: &[(&str, &str)]) -> crate::Result<EvalCtx> {
    let variables = variables
      .iter()
      .map(|(k, v)| (k.to_string(), EnvVariable::String(v.to_string())))
      .collect::<HashMap<_, _>>();

    EvalCtx::new(variables)
  }

  fn eval(ctx: &EvalCtx, src: &str) -> crate::Result<String> {
    let tokens = RequestTokenizer::new(src).tokenize();
    ctx.eval(tokens)
  }

  #[test]
  fn test_evaluates_plain_string() {
    let ctx = eval_ctx(&[]).unwrap();

    assert_eq!(
      eval(&ctx, "just a plain string").unwrap(),
      "just a plain string"
    );
  }

  #[test]
  fn test_evaluates_var_expression() {
    let ctx = eval_ctx(&[("name", "John")]).unwrap();

    assert_eq!(
      eval(&ctx, "{\n  \"name\": ${env.name}\n}").unwrap(),
      "{\n  \"name\": John\n}"
    );
  }

  #[test]
  fn test_var_missing_returns_null() {
    let ctx = eval_ctx(&[]).unwrap();

    assert_eq!(eval(&ctx, "${env.missing}").unwrap(), "null");
  }

  #[test]
  fn test_evaluates_number_var_expression() {
    let ctx = EvalCtx::new(HashMap::from([(
      "age".to_string(),
      EnvVariable::Number(42.0),
    )]))
    .unwrap();

    assert_eq!(eval(&ctx, "${env.age}").unwrap(), "42");
  }

  #[test]
  fn test_evaluates_object_var_expression() {
    let object = EnvVariable::Object(HashMap::from([(
      "city".to_string(),
      EnvVariable::String("NYC".to_string()),
    )]));
    let ctx = EvalCtx::new(HashMap::from([("address".to_string(), object)])).unwrap();

    assert_eq!(eval(&ctx, "${env.address}").unwrap(), "{\"city\":NYC}");
  }

  #[test]
  fn test_evaluates_arithmetic_expression() {
    let ctx = eval_ctx(&[]).unwrap();

    assert_eq!(eval(&ctx, "${1 + 2}").unwrap(), "3");
  }

  #[test]
  fn test_evaluates_boolean_expression() {
    let ctx = eval_ctx(&[]).unwrap();

    assert_eq!(eval(&ctx, "${1 == 1}").unwrap(), "true");
  }

  #[test]
  fn test_evaluates_table_expression() {
    let ctx = eval_ctx(&[]).unwrap();

    assert_eq!(
      eval(&ctx, "${ {a = 1, b = \"x\"} }").unwrap(),
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
}

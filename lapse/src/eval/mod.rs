use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::parsing::RequestToken;
use mlua::{FromLua, Lua, Value};

pub struct EvalCtx {
  variables: Rc<RefCell<HashMap<String, String>>>,
  runtime: Lua,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalResult {
  Null,
  Boolean(bool),
  Integer(i64),
  Number(f64),
  String(String),
  Object(HashMap<String, EvalResult>),
}

impl FromLua for EvalResult {
  fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
    match value {
      Value::Nil => Ok(Self::Null),
      Value::Boolean(b) => Ok(Self::Boolean(b)),
      Value::Integer(i) => Ok(Self::Integer(i)),
      Value::Number(n) => Ok(Self::Number(n)),
      Value::String(s) => Ok(Self::String(s.to_str()?.to_string())),
      Value::Table(table) => {
        let mut object = HashMap::new();
        for pair in table.pairs::<String, Value>() {
          let (key, value) = pair?;
          object.insert(key, Self::from_lua(value, _lua)?);
        }

        Ok(Self::Object(object))
      }
      other => Err(mlua::Error::FromLuaConversionError {
        from: other.type_name(),
        to: "EvalResult".to_owned(),
        message: None,
      }),
    }
  }
}

// renders the result the same way it was written into the request body, e.g. a
// Lua string becomes a quoted JSON string, a table becomes a JSON object
impl fmt::Display for EvalResult {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Null => write!(f, "null"),
      Self::Boolean(b) => write!(f, "{b}"),
      Self::Integer(i) => write!(f, "{i}"),
      Self::Number(n) => write!(f, "{n}"),
      Self::String(s) => write!(f, "{}", serde_json::to_string(s).unwrap()),
      Self::Object(fields) => {
        let mut keys: Vec<_> = fields.keys().collect();
        keys.sort();

        write!(f, "{{")?;
        for (i, key) in keys.into_iter().enumerate() {
          if i > 0 {
            write!(f, ",")?;
          }
          write!(f, "{}:{}", serde_json::to_string(key).unwrap(), fields[key])?;
        }
        write!(f, "}}")
      }
    }
  }
}

impl EvalCtx {
  pub fn new(variables: HashMap<String, String>) -> Self {
    let variables = Rc::new(RefCell::new(variables));
    let runtime = Lua::new();

    let lookup = Rc::clone(&variables);
    let var_fn = runtime
      .create_function(move |_, name: String| Ok(lookup.borrow().get(&name).cloned()))
      .unwrap();

    runtime.globals().set("var", var_fn).unwrap();

    Self { variables, runtime }
  }

  pub fn set_variable(&self, name: impl Into<String>, value: impl Into<String>) {
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
          let value: EvalResult = self.runtime.load(inner).eval().unwrap();
          result.push_str(&value.to_string());
        }
      }
    }

    result
  }
}

#[cfg(test)]
mod test {
  use super::{EvalCtx, EvalResult};
  use crate::parsing::RequestTokenizer;
  use std::collections::HashMap;

  fn eval_ctx(variables: &[(&str, &str)]) -> EvalCtx {
    let variables = variables
      .iter()
      .map(|(k, v)| (k.to_string(), v.to_string()))
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
      "{\n  \"name\": \"John\"\n}"
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

    assert_eq!(eval(&ctx, "${var(\"name\")}"), "\"Jane\"");
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

    assert_eq!(
      eval(&ctx, "${ {a = 1, b = \"x\"} }"),
      "{\"a\":1,\"b\":\"x\"}"
    );
  }

  #[test]
  fn test_eval_result_display() {
    assert_eq!(EvalResult::Null.to_string(), "null");
    assert_eq!(EvalResult::Boolean(true).to_string(), "true");
    assert_eq!(EvalResult::Integer(42).to_string(), "42");
    assert_eq!(EvalResult::String("hi".to_owned()).to_string(), "\"hi\"");
  }
}

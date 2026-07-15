use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::{env::EnvVariable, parsing::RequestToken};
use mlua::{FromLua, IntoLua, Lua, Value};

pub struct EvalCtx {
  variables: Rc<RefCell<HashMap<String, EnvVariable>>>,
  runtime: Lua,
}

impl FromLua for EnvVariable {
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
        to: "EnvVariable".to_owned(),
        message: None,
      }),
    }
  }
}
impl IntoLua for EnvVariable {
  fn into_lua(self, lua: &Lua) -> mlua::prelude::LuaResult<Value> {
    match self {
      Self::Null => Ok(Value::Nil),
      Self::Boolean(b) => Ok(Value::Boolean(b)),
      Self::Integer(i) => Ok(Value::Integer(i)),
      Self::Number(n) => Ok(Value::Number(n)),
      Self::String(s) => Ok(Value::String(lua.create_string(s)?)),
      Self::Object(fields) => {
        let table = lua.create_table()?;
        for (key, value) in fields {
          table.set(key, value.into_lua(lua)?)?;
        }

        Ok(Value::Table(table))
      }
    }
  }
}

// renders the result the same way it was written into the request body, e.g. a
// Lua string becomes a quoted JSON string, a table becomes a JSON object
impl fmt::Display for EnvVariable {
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

    assert_eq!(eval(&ctx, "${var(\"address\")}"), "{\"city\":\"NYC\"}");
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
  fn test_env_variable_display() {
    assert_eq!(EnvVariable::Null.to_string(), "null");
    assert_eq!(EnvVariable::Boolean(true).to_string(), "true");
    assert_eq!(EnvVariable::Integer(42).to_string(), "42");
    assert_eq!(EnvVariable::String("hi".to_owned()).to_string(), "\"hi\"");
  }
}

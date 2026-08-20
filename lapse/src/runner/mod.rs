pub mod http;
use std::{collections::HashMap, fmt::Display};

use mlua::{IntoLua, Lua, Value};

use crate::{
  env::{EnvValue, hook::Event},
  lua::lexer::{DocumentLexer, DocumentToken},
};

pub struct RequestRunner {
  runtime: Lua,
  hooks: HashMap<Event, Vec<String>>,
  default_scheme: String,
}

#[derive(Clone)]
pub struct Response {
  pub text: String,
  pub status: u16,
  pub headers: HashMap<String, String>,
  pub timestamp: u128,
  pub duration: u128,
}

impl IntoLua for Response {
  fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
    let table = lua.create_table()?;

    table.set("status", self.status)?;
    table.set("text", self.text)?;
    table.set("headers", self.headers)?;

    Ok(Value::Table(table))
  }
}

impl Display for Response {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{}", self.status)?;
    for (header, value) in &self.headers {
      writeln!(f, "{}: {}", header, value)?;
    }
    writeln!(f)?;
    writeln!(f, "{}", self.text)
  }
}

impl RequestRunner {
  pub fn new(runtime: Lua, hooks: HashMap<Event, Vec<String>>, default_scheme: String) -> Self {
    Self {
      runtime,
      hooks,
      default_scheme,
    }
  }

  pub fn eval(&self, doc: &str) -> crate::Result<String> {
    let mut lexer = DocumentLexer::new(doc);
    let tokens = lexer.tokenize();
    let mut result = String::new();

    for token in tokens {
      match token {
        DocumentToken::String(inner) => {
          result.push_str(&inner);
        }
        DocumentToken::Expr(inner) => {
          let value: EnvValue = self.runtime.load(inner).eval()?;
          result.push_str(&value.to_string());
        }
      }
    }

    Ok(result)
  }
}

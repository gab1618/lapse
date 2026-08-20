pub mod eval;
pub mod http;
pub mod runtime;

use std::{collections::HashMap, fmt::Display};

use mlua::{IntoLua, Lua, Value};

use crate::env::hook::Event;

pub struct Runner {
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
    table.set("timestamp", self.timestamp)?;
    table.set("duration", self.duration)?;

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

impl Runner {
  pub fn new(runtime: Lua, hooks: HashMap<Event, Vec<String>>, default_scheme: String) -> Self {
    Self {
      runtime,
      hooks,
      default_scheme,
    }
  }
}

pub mod lexer;

#[cfg(test)]
mod test;

use std::{collections::HashMap, fmt};

use mlua::{FromLua, IntoLua, Lua, Value};

use crate::{env::EnvValue, log::ResponseLog};

impl FromLua for EnvValue {
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
impl IntoLua for EnvValue {
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

impl fmt::Display for EnvValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Null => write!(f, "null"),
      Self::Boolean(b) => write!(f, "{b}"),
      Self::Integer(i) => write!(f, "{i}"),
      Self::Number(n) => write!(f, "{n}"),
      Self::String(s) => write!(f, "{}", s),
      Self::Object(fields) => {
        let mut keys: Vec<_> = fields.keys().collect();
        keys.sort();

        write!(f, "{{")?;
        for (i, key) in keys.into_iter().enumerate() {
          if i > 0 {
            write!(f, ",")?;
          }
          write!(
            f,
            "{}:{}",
            serde_json::to_string(key).map_err(|_| Default::default())?,
            fields[key]
          )?;
        }
        write!(f, "}}")
      }
    }
  }
}

impl IntoLua for ResponseLog {
  fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
    let table = lua.create_table()?;

    table.set("status", self.status)?;
    table.set("text", self.text)?;
    table.set("headers", self.headers)?;
    table.set("request", self.request)?;
    table.set("duration", self.duration)?;
    table.set("timestamp", self.timestamp)?;

    Ok(Value::Table(table))
  }
}

use std::{collections::HashMap, fmt};

use mlua::{FromLua, IntoLua, Lua, Value as LuaValue};

#[derive(PartialEq, Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Value {
  Null,
  Boolean(bool),
  Integer(i64),
  Number(f64),
  String(String),
  Object(HashMap<String, Value>),
}

impl From<bool> for Value {
  fn from(value: bool) -> Self {
    Self::Boolean(value)
  }
}
impl From<f64> for Value {
  fn from(value: f64) -> Self {
    Self::Number(value)
  }
}
impl From<i64> for Value {
  fn from(value: i64) -> Self {
    Self::Integer(value)
  }
}
impl From<String> for Value {
  fn from(value: String) -> Self {
    Self::String(value)
  }
}
impl From<&str> for Value {
  fn from(value: &str) -> Self {
    value.to_string().into()
  }
}
impl From<HashMap<String, Value>> for Value {
  fn from(value: HashMap<String, Value>) -> Self {
    Self::Object(value)
  }
}

impl fmt::Display for Value {
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

impl IntoLua for Value {
  fn into_lua(self, lua: &Lua) -> mlua::prelude::LuaResult<LuaValue> {
    match self {
      Self::Null => Ok(LuaValue::Nil),
      Self::Boolean(b) => Ok(LuaValue::Boolean(b)),
      Self::Integer(i) => Ok(LuaValue::Integer(i)),
      Self::Number(n) => Ok(LuaValue::Number(n)),
      Self::String(s) => Ok(LuaValue::String(lua.create_string(s)?)),
      Self::Object(fields) => {
        let table = lua.create_table()?;
        for (key, value) in fields {
          table.set(key, value.into_lua(lua)?)?;
        }

        Ok(LuaValue::Table(table))
      }
    }
  }
}

impl FromLua for Value {
  fn from_lua(value: LuaValue, _lua: &Lua) -> mlua::Result<Self> {
    match value {
      LuaValue::Nil => Ok(Self::Null),
      LuaValue::Boolean(b) => Ok(Self::Boolean(b)),
      LuaValue::Integer(i) => Ok(Self::Integer(i)),
      LuaValue::Number(n) => Ok(Self::Number(n)),
      LuaValue::String(s) => Ok(Self::String(s.to_str()?.to_string())),
      LuaValue::Table(table) => {
        let mut object = HashMap::new();
        for pair in table.pairs::<String, LuaValue>() {
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

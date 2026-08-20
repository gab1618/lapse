#[cfg(test)]
mod test;

use mlua::{IntoLua, Lua, Value};

use crate::log::ResponseLog;

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

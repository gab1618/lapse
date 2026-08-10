pub mod error;

use std::{fs, sync::Arc};

use mlua::{Lua, UserData, UserDataMethods};

use crate::{Lapse, script::error::ScriptError};

struct LapseLuaApi {
  lapse: Arc<Lapse>,
}

impl LapseLuaApi {
  pub fn new(lapse: Arc<Lapse>) -> Self {
    Self { lapse }
  }
}

impl UserData for LapseLuaApi {
  fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
    methods.add_async_method_mut("request", |lua, this, name: String| async move {
      this
        .lapse
        .request_with(&name, lua, Default::default())
        .await
        .map_err(|_| mlua::Error::RuntimeError("Error when executing request".to_string()))
    });
  }
}

impl Lapse {
  pub fn get_runtime(&self) -> crate::Result<Lua> {
    let runtime = Lua::new();

    let shared_lapse = Arc::new(self.clone());

    let lapse_api = LapseLuaApi::new(shared_lapse);

    let env = self.get_env(&self.current_env())?;

    runtime.globals().set("env", env.variables)?;
    runtime.globals().set("secret", env.secrets)?;

    runtime.globals().set("lapse", lapse_api)?;

    Ok(runtime)
  }
  pub async fn run_script(&self, name: &str) -> crate::Result<()> {
    let script_path = self.scripts_path().join(name).with_extension("lua");

    let script_content = fs::read_to_string(script_path).map_err(ScriptError::ReadScriptFile)?;
    let runtime = self.get_runtime()?;

    let loaded = runtime.load(script_content);
    loaded.exec_async().await?;

    Ok(())
  }
}

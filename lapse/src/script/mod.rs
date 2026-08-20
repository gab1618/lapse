pub mod error;

use std::{fs, ops::Deref};

use mlua::{Lua, UserData, UserDataMethods};

use crate::{Lapse, request::runner::RequestRunner, script::error::ScriptError};

struct LapseLuaApi(pub Lapse);
impl Deref for LapseLuaApi {
  type Target = Lapse;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl UserData for LapseLuaApi {
  fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
    methods.add_async_method_mut("request", |lua, this, name: String| async move {
      let curr_env_name = this.current_env();
      let curr_env = this.get_env(&curr_env_name).unwrap();
      let runner = RequestRunner::new(
        lua,
        Default::default(),
        curr_env.config.default_scheme.to_string(),
      );

      let req = this.get_raw_request_http(&name).unwrap();

      runner
        .execute(&req)
        .await
        .map_err(|_| mlua::Error::RuntimeError("Error when executing request".to_string()))
    });
  }
}

impl Lapse {
  pub fn get_runtime(&self) -> crate::Result<Lua> {
    let runtime = Lua::new();

    let lapse_api = LapseLuaApi(self.clone());

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

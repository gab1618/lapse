use std::{fs, sync::Arc};

use mlua::{Lua, UserData, UserDataMethods};

use crate::Lapse;

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
    methods.add_async_method("request", |_, this, name: String| async move {
      let response = this
        .lapse
        .request(name)
        .await
        .map_err(|_| mlua::Error::RuntimeError("Error when executing request".to_string()))?;

      Ok(response)
    });
  }
}

impl Lapse {
  fn get_runtime(&self) -> crate::Result<Lua> {
    let runtime = Lua::new();

    let shared_lapse = Arc::new(self.clone());

    let lapse_api = LapseLuaApi::new(shared_lapse);

    let variables = self
      .current_env()
      .ok()
      .flatten()
      .map(|name| self.get_env(&name).unwrap_or_default())
      .unwrap_or_default();

    let secrets = self.load_secrets().unwrap_or_default();

    runtime.globals().set("env", variables)?;
    runtime.globals().set("secret", secrets)?;

    runtime.globals().set("lapse", lapse_api)?;

    Ok(runtime)
  }
  pub async fn run_script(&self, name: &str) -> crate::Result<()> {
    let script_path = self.scripts_path().join(name).with_extension("lua");

    let script_content = fs::read_to_string(script_path).unwrap();
    let runtime = self.get_runtime()?;

    let loaded = runtime.load(script_content);
    loaded.exec_async().await?;

    Ok(())
  }
}

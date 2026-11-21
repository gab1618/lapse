use mlua::{UserData, UserDataMethods};

use crate::{Lapse, env::config::EnvConfig, runner::Runner};

pub struct LapseLuaApi {
  lapse: Lapse,
  config: EnvConfig,
}

impl LapseLuaApi {
  pub fn new(lapse: Lapse, config: EnvConfig) -> Self {
    Self { lapse, config }
  }
}

impl UserData for LapseLuaApi {
  fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
    methods.add_method("get_request", |_, this, name: String| {
      this
        .lapse
        .get_raw_request_http(&name)
        .map_err(|_| mlua::Error::RuntimeError(format!("Could not find request {}", name)))
    });
    methods.add_async_method_mut("request", |lua, this, req: String| async move {
      let runner = Runner::new(
        lua,
        Default::default(),
        this.config.default_scheme.to_string(),
      );

      runner
        .execute(&req)
        .await
        .map_err(|_| mlua::Error::RuntimeError("Error when executing request".to_string()))
    });
  }
}

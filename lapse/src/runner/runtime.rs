use mlua::Lua;

use crate::{Lapse, runner::Runner};

use std::{fs, ops::Deref};

use mlua::{UserData, UserDataMethods};

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
      let curr_env = this.get_env(&curr_env_name).ok().unwrap_or_default();

      let runner = Runner::new(
        lua,
        Default::default(),
        curr_env.config.default_scheme.to_string(),
      );

      let req = this
        .get_raw_request_http(&name)
        .map_err(|_| mlua::Error::RuntimeError(format!("Could not find request {}", name)))?;

      runner
        .execute(&req)
        .await
        .map_err(|_| mlua::Error::RuntimeError("Error when executing request".to_string()))
    });
  }
}

impl Runner {
  pub fn standalone() -> Self {
    Self {
      runtime: Lua::new(),
      hooks: Default::default(),
      default_scheme: "http://".to_string(),
    }
  }
  pub fn from_space(space: &Lapse) -> crate::Result<Self> {
    let runtime = Lua::new();

    let lapse_api = LapseLuaApi(space.clone());

    let env = space.get_env(&space.current_env())?;

    runtime.globals().set("Env", env.variables)?;
    runtime.globals().set("Secret", env.secrets)?;

    runtime.globals().set("Lapse", lapse_api)?;

    let hooks = env
      .hooks
      .into_iter()
      .filter_map(|(event, entry)| {
        if !entry.enabled {
          return None;
        }
        let scripts = entry
          .scripts
          .iter()
          .map(|entry| space.scripts_path().join(entry))
          .filter_map(|path| fs::read_to_string(path).ok())
          .collect::<Vec<String>>();

        Some((event, scripts))
      })
      .collect();

    Ok(Self {
      runtime,
      hooks,
      default_scheme: env.config.default_scheme.to_string(),
    })
  }

  pub async fn run(&self, content: &str) -> crate::Result<()> {
    let loaded = self.runtime.load(content);
    loaded.exec_async().await?;

    Ok(())
  }
}

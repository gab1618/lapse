use crate::{Lapse, runner::Runner};
use mlua::Lua;
use std::fs;

use lapse::LapseLuaApi;

mod lapse;

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

    let mut env = space.get_env(&space.current_env().unwrap_or_default())?;

    runtime.globals().set("Env", env.variables)?;
    runtime.globals().set("Secret", env.secrets)?;

    let default_scheme = env.config.default_scheme.to_string();

    // Config is useless, now that we took the default scheme
    let config = std::mem::take(&mut env.config);

    let lapse_api = LapseLuaApi::new(space.clone(), config);
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
      default_scheme,
    })
  }

  pub async fn run(&self, content: &str) -> crate::Result<()> {
    let loaded = self.runtime.load(content);
    loaded.exec_async().await?;

    Ok(())
  }
}

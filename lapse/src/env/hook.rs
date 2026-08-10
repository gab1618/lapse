use std::{collections::HashMap, fs};

use crate::Lapse;

#[cfg_attr(test, derive(Debug))]
#[derive(serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
pub enum Event {
  #[serde(rename = "pre-request")]
  PreRequest,
  #[serde(rename = "post-request")]
  PostRequest,
}

#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct HookEntry {
  pub enabled: bool,
  pub scripts: Vec<String>,
}

impl Lapse {
  pub fn get_hooks_scripts(&self) -> crate::Result<HashMap<Event, Vec<String>>> {
    let curr_env = self.current_env();
    let env = self.get_env(&curr_env)?;

    let hooks = env.hooks;

    let enabled_hooks = hooks
      .into_iter()
      .filter_map(|(event, entry)| {
        if !entry.enabled {
          return None;
        }

        let scripts = entry
          .scripts
          .iter()
          .map(|entry| self.scripts_path().join(entry))
          .filter_map(|path| fs::read_to_string(path).ok())
          .collect::<Vec<String>>();

        Some((event, scripts))
      })
      .collect::<HashMap<Event, Vec<String>>>();

    Ok(enabled_hooks)
  }
}

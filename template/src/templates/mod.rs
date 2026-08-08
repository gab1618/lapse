use std::{
  fs,
  path::{Path, PathBuf},
};

use crate::error::Error;

#[cfg(test)]
mod test;

pub mod default;
pub mod httpbin;

pub struct TemplateEntry {
  name: String,
  content: String,
}

pub enum TemplateItem {
  Template(TemplateEntry),
  Collection(TemplateCollection),
}

impl From<TemplateEntry> for TemplateItem {
  fn from(value: TemplateEntry) -> Self {
    Self::Template(value)
  }
}

impl From<TemplateCollection> for TemplateItem {
  fn from(value: TemplateCollection) -> Self {
    Self::Collection(value)
  }
}

#[derive(Default)]
pub struct TemplateCollection {
  name: String,
  items: Vec<TemplateItem>,
}

impl From<Vec<TemplateItem>> for TemplateCollection {
  fn from(value: Vec<TemplateItem>) -> Self {
    Self {
      name: Default::default(),
      items: value,
    }
  }
}

impl TemplateCollection {
  pub fn load<P: AsRef<Path>>(&self, base: P) -> crate::Result<()> {
    let base = base.as_ref().join(&self.name);
    fs::create_dir_all(&base).map_err(Error::CreateCollection)?;

    for entry in self.items.iter() {
      match entry {
        TemplateItem::Template(entry) => {
          let entry_path = base.join(&entry.name);
          fs::write(entry_path, &entry.content).map_err(Error::CreateTemplateFile)?;
        }
        TemplateItem::Collection(coll) => {
          coll.load(&base)?;
        }
      }
    }

    Ok(())
  }
}

/// Presets are templates that can be used to initialize spaces. Since they import multiple types of
/// resources, we can't use them to import into existing spaces, only initialize.
pub struct LapsePreset {
  scripts: TemplateCollection,
  requests: TemplateCollection,
}

impl LapsePreset {
  pub fn load(&self, base: PathBuf) -> crate::Result<()> {
    self.scripts.load(base.join("scripts"))?;
    self.requests.load(base.join("requests"))?;

    Ok(())
  }
}

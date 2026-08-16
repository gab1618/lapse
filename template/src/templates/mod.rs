use std::{fs, ops, path::Path};

use crate::error::Error;

#[cfg(test)]
mod test;

pub mod default;
pub mod httpbin;
pub mod openapi;
pub mod request_file;

pub enum TemplateEntry {
  File(String, String),
  Dir(String),
}

/// Presets are templates that can be used to initialize spaces. Since they import multiple types of
/// resources, we can't use them to import into existing spaces, only initialize.
pub struct LapsePreset {
  entries: Vec<TemplateEntry>,
}

impl LapsePreset {
  pub fn new(entries: Vec<TemplateEntry>) -> Self {
    Self { entries }
  }
  pub fn load_empty<P: AsRef<Path>>(&self, path: P) -> crate::Result<()> {
    let base = path.as_ref();

    fs::create_dir_all(base.join(".lapse")).map_err(crate::Error::CreateTemplateFile)?;
    fs::create_dir_all(base.join("requests")).map_err(crate::Error::CreateTemplateFile)?;
    fs::create_dir_all(base.join("env/default")).map_err(crate::Error::CreateTemplateFile)?;
    fs::create_dir_all(base.join("scripts")).map_err(crate::Error::CreateTemplateFile)?;

    Ok(())
  }
  pub fn load<P: AsRef<Path>>(&self, path: P) -> crate::Result<()> {
    let base = path.as_ref();

    self.load_empty(base)?;

    for entry in self.entries.iter() {
      match entry {
        TemplateEntry::File(name, content) => {
          fs::write(base.join(name), content).map_err(Error::CreateTemplateFile)?
        }
        TemplateEntry::Dir(name) => {
          fs::create_dir_all(base.join(name)).map_err(Error::CreateCollection)?
        }
      }
    }

    Ok(())
  }
}

impl ops::Add for LapsePreset {
  type Output = LapsePreset;

  /// Adds together two presets
  fn add(mut self, rhs: Self) -> Self::Output {
    for entry in rhs.entries {
      self.entries.push(entry);
    }

    self
  }
}

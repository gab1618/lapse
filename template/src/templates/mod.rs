use std::{fs, io::Write as _, path::PathBuf};

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

pub struct TemplateCollection {
  name: String,
  items: Vec<TemplateItem>,
}

impl TemplateCollection {
  pub fn load(&self, base: PathBuf) -> crate::Result<()> {
    for entry in self.items.iter() {
      match entry {
        TemplateItem::Template(entry) => {
          let entry_path = base.join(&entry.name);
          fs::write(entry_path, &entry.content).map_err(Error::CreateTemplateFile)?;
        }
        TemplateItem::Collection(coll) => {
          let dir_path = base.join(&coll.name);
          fs::create_dir(&dir_path).map_err(Error::CreateTemplateFile)?;

          coll.load(dir_path)?;
        }
      }
    }

    Ok(())
  }
}

/// Presets are templates that can be used to initialize spaces. Since they import multiple types of
/// resources, we can't use them to import into existing spaces, only initialize.
pub struct LapsePreset {
  scripts: Vec<TemplateItem>,
  requests: Vec<TemplateItem>,
}

impl LapsePreset {
  fn load_templates(base: PathBuf, templates: &[TemplateItem]) -> crate::Result<()> {
    for template in templates {
      match template {
        TemplateItem::Template(template_entry) => {
          let mut f =
            fs::File::create(base.join(&template_entry.name)).map_err(Error::CreateTemplateFile)?;
          write!(f, "{}", template_entry.content).map_err(Error::CreateTemplateFile)?;
        }
        TemplateItem::Collection(coll) => {
          coll.load(base.join(&coll.name))?;
        }
      }
    }

    Ok(())
  }
  pub fn load(&self, base: PathBuf) -> crate::Result<()> {
    Self::load_templates(base.join("scripts"), &self.scripts)?;
    Self::load_templates(base.join("requests"), &self.requests)?;

    Ok(())
  }
}

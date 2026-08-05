use std::{fs, io::Write as _, path::PathBuf};

#[cfg(test)]
mod test;

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
  pub fn load(&self, base: PathBuf) {
    for entry in self.items.iter() {
      match entry {
        TemplateItem::Template(entry) => {
          let entry_path = base.join(&entry.name);
          fs::write(entry_path, &entry.content).unwrap();
        }
        TemplateItem::Collection(coll) => {
          let dir_path = base.join(&coll.name);
          fs::create_dir(&dir_path).unwrap();

          coll.load(dir_path);
        }
      }
    }
  }
}

pub struct LapseTemplate {
  scripts: Vec<TemplateItem>,
  requests: Vec<TemplateItem>,
}

impl LapseTemplate {
  fn load_templates(base: PathBuf, templates: &[TemplateItem]) {
    for template in templates {
      match template {
        TemplateItem::Template(template_entry) => {
          let mut f = fs::File::create(base.join(&template_entry.name)).unwrap();
          write!(f, "{}", template_entry.content).unwrap();
        }
        TemplateItem::Collection(coll) => {
          coll.load(base.join(&coll.name));
        }
      }
    }
  }
  pub fn load(&self, base: PathBuf) {
    Self::load_templates(base.join("scripts"), &self.scripts);
    Self::load_templates(base.join("requests"), &self.requests);
  }
}

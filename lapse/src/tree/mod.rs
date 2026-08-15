pub mod error;
pub mod resource;

#[cfg(test)]
mod test;

use std::{fs, ops::Deref, path::Path};

use crate::{
  Lapse,
  tree::{error::TreeError, resource::Resource},
};

pub enum TreeEntry {
  Entry(String),
  Subtree(String, Tree),
}

pub struct Tree(Vec<TreeEntry>);

impl Deref for Tree {
  type Target = Vec<TreeEntry>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Clone, Copy)]
pub struct FlatTreeConfig {
  pub files: bool,
  pub dirs: bool,
}

impl FlatTreeConfig {
  pub fn include_files(mut self, val: bool) -> Self {
    self.files = val;
    self
  }
  pub fn include_dirs(mut self, val: bool) -> Self {
    self.dirs = val;
    self
  }
}

impl Default for FlatTreeConfig {
  fn default() -> Self {
    Self {
      files: true,
      dirs: Default::default(),
    }
  }
}

impl Tree {
  pub fn read(prefix: &Path, path: &Path) -> crate::Result<Self> {
    let mut dir_entries = fs::read_dir(path)
      .map_err(TreeError::ReadDir)?
      .map(|entry| {
        let resolved_entry = entry.map_err(TreeError::ReadDir)?;
        Ok(resolved_entry)
      })
      .collect::<crate::Result<Vec<_>>>()?;

    dir_entries.sort_by_key(|entry| entry.file_name());

    Ok(Self(
      dir_entries
        .into_iter()
        .map(|entry| {
          let full_entry_path = entry.path();

          if full_entry_path.is_dir() {
            let name = entry.file_name().to_string_lossy().into_owned();
            let sub_tree = Self::read(prefix, &full_entry_path)?;
            Ok(TreeEntry::Subtree(name, sub_tree))
          } else {
            let relative_path = full_entry_path
              .strip_prefix(prefix)
              .map_err(|_| TreeError::ParseTreePath)?
              .with_extension("")
              .to_str()
              .ok_or(TreeError::ParseTreePath)?
              .to_owned();
            Ok(TreeEntry::Entry(relative_path))
          }
        })
        .collect::<crate::Result<Vec<_>>>()?,
    ))
  }
  pub fn as_flat(&self, config: FlatTreeConfig) -> Vec<String> {
    let mut entries: Vec<String> = vec![];

    for entry in self.iter() {
      match entry {
        TreeEntry::Entry(entry) => {
          if config.files {
            entries.push(entry.clone());
          }
        }
        TreeEntry::Subtree(entry, items) => {
          if config.dirs {
            entries.push(entry.clone());
          }
          let sub_requests = items.as_flat(config);
          for sub in sub_requests.into_iter() {
            entries.push(sub);
          }
        }
      }
    }

    entries
  }
}

impl Lapse {
  pub fn get_resource_tree(&self, resource: Resource, base: Option<String>) -> crate::Result<Tree> {
    let resource_subpath: &str = resource.into();
    let resource_path = self.path.join(resource_subpath);

    let dir = match base {
      Some(base) => resource_path.join(base),
      None => resource_path.clone(),
    };

    Tree::read(&resource_path, &dir)
  }
}

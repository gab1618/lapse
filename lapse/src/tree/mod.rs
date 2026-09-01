pub mod error;
pub mod resource;

use std::{
  fs,
  ops::Deref,
  path::{Path, PathBuf},
};

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

pub enum TraverseEntryKind {
  Entry,
  Subtree,
}

pub struct TraverseEntry {
  pub name: String,
  pub kind: TraverseEntryKind,
  pub depth: usize,
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
            let sub_tree = Self::read(&prefix.join(&name), &full_entry_path)?;
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
          let sub_entries = items.as_flat(config);
          for sub in sub_entries.into_iter() {
            entries.push(format!("{entry}/{sub}"));
          }
        }
      }
    }

    entries
  }
  pub fn traverse<F: Fn(TraverseEntry)>(&self, parent_name: String, depth: usize, f: &F) {
    let prefix = if parent_name.is_empty() {
      Default::default()
    } else {
      format!("{}/", parent_name)
    };

    for entry in self.iter() {
      match entry {
        TreeEntry::Entry(name) => {
          let entry = TraverseEntry {
            name: format!("{prefix}{name}"),
            kind: TraverseEntryKind::Entry,
            depth,
          };

          f(entry)
        }
        TreeEntry::Subtree(name, tree) => {
          let entry = TraverseEntry {
            name: format!("{prefix}{name}"),
            kind: TraverseEntryKind::Subtree,
            depth,
          };
          f(entry);
          tree.traverse(format!("{prefix}{name}"), depth + 1, f);
        }
      }
    }
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
  pub fn resource_path(&self, resource: Resource, name: &str) -> crate::Result<PathBuf> {
    let resource_subpath: &str = resource.into();
    let resource_path = self.path.join(resource_subpath).join(name);

    Ok(resource_path)
  }
}

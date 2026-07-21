pub mod error;

use std::{fs, ops::Deref, path::Path};

use crate::{Lapse, tree::error::TreeError};

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
}

impl Lapse {
  pub fn get_resource_tree(&self, resource: &str, base: Option<String>) -> crate::Result<Tree> {
    let resource_path = self.path.join(resource);

    let dir = match base {
      Some(base) => resource_path.join(base),
      None => resource_path.clone(),
    };

    Tree::read(&resource_path, &dir)
  }
}

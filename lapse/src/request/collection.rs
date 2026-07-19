use std::{fs, path::Path};

use crate::{Lapse, request::error::RequestError};

pub enum RequestsCollectionEntry {
  Request(String),
  Collection(String, Box<RequestCollection>),
}

pub type RequestCollection = Vec<RequestsCollectionEntry>;

impl Lapse {
  pub fn read_collection(prefix: &Path, dir: &Path) -> crate::Result<RequestCollection> {
    let mut dir_entries = fs::read_dir(dir)
      .map_err(RequestError::ReadCollectionDir)?
      .map(|entry| {
        let resolved_entry = entry.map_err(RequestError::ReadCollectionDir)?;
        Ok(resolved_entry)
      })
      .collect::<crate::Result<Vec<_>>>()?;

    dir_entries.sort_by_key(|entry| entry.file_name());

    dir_entries
      .into_iter()
      .map(|entry| {
        let full_entry_path = entry.path();

        if full_entry_path.is_dir() {
          let name = entry.file_name().to_string_lossy().into_owned();
          let sub_collection = Self::read_collection(prefix, &full_entry_path)?;
          Ok(RequestsCollectionEntry::Collection(
            name,
            Box::new(sub_collection),
          ))
        } else {
          let relative_path = full_entry_path
            .strip_prefix(prefix)
            .map_err(|_| RequestError::ParseCollectionPath)?
            .with_extension("")
            .to_str()
            .ok_or(RequestError::ParseCollectionPath)?
            .to_owned();
          Ok(RequestsCollectionEntry::Request(relative_path))
        }
      })
      .collect::<crate::Result<Vec<_>>>()
  }
}

#[cfg(test)]
mod test {
  use std::fs;

  use tempfile::tempdir;

  use crate::{
    Lapse,
    request::collection::{RequestCollection, RequestsCollectionEntry},
  };

  fn entry_names(collection: &RequestCollection) -> Vec<String> {
    let mut names = collection
      .iter()
      .map(|entry| match entry {
        RequestsCollectionEntry::Request(path) => path.clone(),
        RequestsCollectionEntry::Collection(name, _) => name.clone(),
      })
      .collect::<Vec<_>>();
    names.sort();
    names
  }

  #[test]
  fn test_get_flat_collection() {
    let temp_dir = tempdir().unwrap();
    let lapse = Lapse::init(temp_dir.path()).unwrap();

    fs::write(temp_dir.path().join("requests").join("ping.md"), "").unwrap();

    let collection = lapse.get_request_collection(None).unwrap();

    assert_eq!(entry_names(&collection), vec!["ping", "request"]);
  }

  #[test]
  fn test_get_nested_collection_strips_prefix() {
    let temp_dir = tempdir().unwrap();
    let lapse = Lapse::init(temp_dir.path()).unwrap();

    let requests_path = temp_dir.path().join("requests");
    fs::create_dir(requests_path.join("users")).unwrap();
    fs::write(requests_path.join("users").join("get.md"), "").unwrap();
    fs::write(requests_path.join("users").join("create.md"), "").unwrap();

    let collection = lapse.get_request_collection(None).unwrap();

    let users_collection = collection
      .iter()
      .find_map(|entry| match entry {
        RequestsCollectionEntry::Collection(name, sub) if name == "users" => Some(sub),
        _ => None,
      })
      .expect("expected a `users` collection entry");

    assert_eq!(
      entry_names(users_collection),
      vec!["users/create", "users/get"]
    );
  }

  #[test]
  fn test_get_collection_with_base_still_returns_full_relative_path() {
    let temp_dir = tempdir().unwrap();
    let lapse = Lapse::init(temp_dir.path()).unwrap();

    let requests_path = temp_dir.path().join("requests");
    fs::create_dir(requests_path.join("users")).unwrap();
    fs::write(requests_path.join("users").join("get.md"), "").unwrap();

    let collection = lapse
      .get_request_collection(Some("users".to_owned()))
      .unwrap();

    assert_eq!(entry_names(&collection), vec!["users/get"]);
  }
}

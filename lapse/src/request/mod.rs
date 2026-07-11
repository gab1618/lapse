use std::{fs, path::Path};

use crate::Lapse;

pub mod http;

pub struct RequestFile {
  pub markdown: String,
  pub http: String,
}

impl Lapse {
  pub fn get_request_file(&self, path: &str) -> RequestFile {
    let file_path = self.requests_path().join(path).with_extension("md");
    let file_content = fs::read_to_string(file_path).unwrap();

    let (markdown, http) = file_content.split_once("---").unwrap();

    RequestFile {
      markdown: markdown.to_owned(),
      http: http.to_owned(),
    }
  }
  pub fn get_request_collection(&self, base: Option<String>) -> RequestCollection {
    let requests_path = self.requests_path();

    let dir = match base {
      Some(base) => requests_path.join(base),
      None => requests_path.clone(),
    };

    Self::read_collection(&requests_path, &dir)
  }

  fn read_collection(prefix: &Path, dir: &Path) -> RequestCollection {
    let mut dir_entries = fs::read_dir(dir)
      .unwrap()
      .map(|entry| entry.unwrap())
      .collect::<Vec<_>>();

    dir_entries.sort_by_key(|entry| entry.file_name());

    dir_entries
      .into_iter()
      .map(|entry| {
        let full_entry_path = entry.path();

        if full_entry_path.is_dir() {
          let name = entry.file_name().to_string_lossy().into_owned();
          let sub_collection = Self::read_collection(prefix, &full_entry_path);
          RequestsCollectionEntry::Collection(name, Box::new(sub_collection))
        } else {
          let relative_path = full_entry_path
            .strip_prefix(prefix)
            .unwrap()
            .with_extension("")
            .to_str()
            .unwrap()
            .to_owned();
          RequestsCollectionEntry::Request(relative_path)
        }
      })
      .collect::<Vec<_>>()
  }
}

pub enum RequestsCollectionEntry {
  Request(String),
  Collection(String, Box<RequestCollection>),
}

pub type RequestCollection = Vec<RequestsCollectionEntry>;

#[cfg(test)]
mod test {
  use std::fs;

  use crate::Lapse;
  use tempfile::tempdir;

  use super::{RequestCollection, RequestsCollectionEntry};

  #[test]
  fn test_get_sample_req() {
    let temp_dir = tempdir().unwrap();
    let lapse = Lapse::init(temp_dir.path()).unwrap();

    let file = lapse.get_request_file("request");
    assert!(!file.markdown.is_empty());
    assert!(!file.http.is_empty());
  }

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

    let collection = lapse.get_request_collection(None);

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

    let collection = lapse.get_request_collection(None);

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

    let collection = lapse.get_request_collection(Some("users".to_owned()));

    assert_eq!(entry_names(&collection), vec!["users/get"]);
  }
}

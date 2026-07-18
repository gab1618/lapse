use crate::{Lapse, request::collection::RequestCollection};
use std::fs;

pub mod collection;
pub mod http;

pub struct RequestFile {
  pub http: String,
  pub markdown: Option<String>,
}

impl Lapse {
  pub fn get_request_file(&self, path: &str) -> crate::Result<RequestFile> {
    let file_path = self.requests_path().join(path).with_extension("md");
    let file_content = fs::read_to_string(file_path).unwrap();

    let (http, markdown) = file_content
      .split_once("---")
      .map(|(inner_http, inner_markdown)| (inner_http, Some(inner_markdown)))
      .unwrap_or_else(|| (&file_content, None));

    Ok(RequestFile {
      markdown: markdown.map(|inner| inner.to_owned()),
      http: http.to_owned(),
    })
  }
  pub fn get_request_collection(&self, base: Option<String>) -> RequestCollection {
    let requests_path = self.requests_path();

    let dir = match base {
      Some(base) => requests_path.join(base),
      None => requests_path.clone(),
    };

    Self::read_collection(&requests_path, &dir)
  }
}

#[cfg(test)]
mod test {
  use std::fs;

  use crate::Lapse;
  use tempfile::tempdir;

  #[test]
  fn test_get_sample_req() {
    let temp_dir = tempdir().unwrap();
    let lapse = Lapse::init(temp_dir.path()).unwrap();

    let file = lapse.get_request_file("request").unwrap();
    assert!(file.markdown.is_some());
    assert!(!file.http.is_empty());
  }
  #[test]
  fn test_get_httponly_req() {
    let temp_dir = tempdir().unwrap();
    let lapse = Lapse::init(temp_dir.path()).unwrap();
    let example_req_content = include_str!("../../assets/without-markdown.md");
    let example_req_path = temp_dir.path().join("requests/without-markdown.md");
    fs::write(example_req_path, example_req_content).unwrap();

    let endpoint = lapse.get_request_file("without-markdown").unwrap();

    assert!(!endpoint.http.is_empty());
    assert!(endpoint.markdown.is_none());
  }
}

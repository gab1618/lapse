use std::fs;

use crate::Lapse;

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
}

#[cfg(test)]
mod test {
  use crate::Lapse;
  use tempfile::tempdir;

  #[test]
  fn test_get_sample_req() {
    let temp_dir = tempdir().unwrap();
    let lapse = Lapse::init(temp_dir.path()).unwrap();

    let file = lapse.get_request_file("request");
    assert!(!file.markdown.is_empty());
    assert!(!file.http.is_empty());
  }
}

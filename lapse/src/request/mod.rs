use crate::{Lapse, request::error::RequestError};
use std::{
  fs::OpenOptions,
  io::{BufRead, BufReader},
};

pub mod error;
pub mod http;
pub mod parsing;

impl Lapse {
  fn get_request_http(&self, name: &str) -> crate::Result<String> {
    let file_path = self.requests_path().join(name).with_extension("md");
    let f = OpenOptions::new()
      .read(true)
      .open(file_path)
      .map_err(RequestError::ReadRequestFile)?;

    let r = BufReader::new(f);

    let lines = r.lines();

    let http_lines = lines.take_while(|line| {
      let is_delimiter = line.as_ref().map(|inner| inner == "---").unwrap_or(true);

      !is_delimiter
    });

    let resolved_lines = http_lines
      .map(|line| {
        let resolved = line.map_err(|_| RequestError::ResolveHttpLine)?;

        Ok(resolved)
      })
      .collect::<crate::Result<Vec<_>>>()?;

    let http_content = resolved_lines.join("\n");

    Ok(http_content)
  }
}

#[cfg(test)]
mod test {
  use std::fs;

  use crate::Lapse;
  use tempfile::tempdir;

  #[test]
  fn test_get_httponly_req() {
    let temp_dir = tempdir().unwrap();
    let lapse = Lapse::init(temp_dir.path()).unwrap();
    let example_req_content = include_str!("../../assets/without-markdown.md");
    let example_req_path = temp_dir.path().join("requests/without-markdown.md");
    fs::write(example_req_path, example_req_content).unwrap();

    let request_http = lapse.get_request_http("without-markdown").unwrap();

    assert!(!request_http.is_empty());
    assert_eq!(request_http.trim(), example_req_content.trim());
  }
}

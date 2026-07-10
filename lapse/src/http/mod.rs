use http::{Method, Request, Uri, Version};
use std::str::FromStr;

use crate::request::RequestFile;

impl RequestFile {
  // TODO: add proper error handling
  pub fn request(&self) -> Request<Vec<u8>> {
    let mut lines = self.http.lines().skip_while(|line| line.is_empty());

    let request_line = lines.next().ok_or("Empty request").unwrap();
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next().ok_or("Missing method").unwrap();
    let uri = request_parts.next().ok_or("Missing URI").unwrap();

    let method = Method::from_str(method).unwrap();
    let uri = Uri::from_str(uri).unwrap();

    let mut request_builder = Request::builder()
      .method(method)
      .uri(uri)
      .version(Version::HTTP_11);

    let mut body_started = false;
    let mut body_lines = vec![];

    for line in lines {
      if !body_started {
        if line.is_empty() {
          body_started = true;
          continue;
        }

        let (name, value) = line.split_once(':').ok_or("Invalid header line").unwrap();
        request_builder = request_builder.header(name.trim(), value.trim());
      } else {
        body_lines.push(line);
      }
    }

    request_builder
      .body(body_lines.join("\n").into_bytes())
      .unwrap()
  }
}

#[cfg(test)]
mod test {
  use http::Method;

  use crate::request::RequestFile;

  fn request_file(http: &str) -> RequestFile {
    RequestFile {
      markdown: String::new(),
      http: http.to_owned(),
    }
  }

  #[test]
  fn test_parses_sample_request() {
    let file = request_file(
      include_str!("../../assets/request.md")
        .split_once("---")
        .unwrap()
        .1,
    );

    let request = file.request();

    assert_eq!(request.method(), Method::POST);
    assert_eq!(request.uri(), "https://example.com/comments");
    assert_eq!(
      request.headers().get("content-type").unwrap(),
      "application/json"
    );
    assert_eq!(
      std::str::from_utf8(request.body()).unwrap(),
      "{\n  \"name\": \"sample\"\n}"
    );
  }

  #[test]
  fn test_parses_request_without_body() {
    let file = request_file("GET https://example.com/comments\ncontent-type: application/json\n");

    let request = file.request();

    assert_eq!(request.method(), Method::GET);
    assert_eq!(request.uri(), "https://example.com/comments");
    assert_eq!(
      request.headers().get("content-type").unwrap(),
      "application/json"
    );
    assert!(request.body().is_empty());
  }

  #[test]
  fn test_parses_request_without_headers_or_body() {
    let file = request_file("DELETE https://example.com/comments\n");

    let request = file.request();

    assert_eq!(request.method(), Method::DELETE);
    assert_eq!(request.uri(), "https://example.com/comments");
    assert!(request.headers().is_empty());
    assert!(request.body().is_empty());
  }

  #[test]
  fn test_ignores_leading_blank_lines() {
    let file = request_file("\n\nPUT https://example.com/comments\n");

    let request = file.request();

    assert_eq!(request.method(), Method::PUT);
    assert_eq!(request.uri(), "https://example.com/comments");
  }
}

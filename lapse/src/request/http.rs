#[cfg(test)]
mod test {
  use crate::request::parsing::{ParsedRequest, parse_request_http};

  #[test]
  fn test_parses_sample_request() {
    let file_http = include_str!("../../assets/request.md")
      .split_once("---")
      .unwrap()
      .0;

    match parse_request_http(file_http).unwrap() {
      ParsedRequest::Http(request) => {
        assert_eq!(request.method, "POST");
        assert_eq!(request.url, "https://example.com/comments");
        assert_eq!(
          request.headers.get("content-type").unwrap(),
          "application/json"
        );
        assert_eq!(request.body.trim(), "{\n  \"name\": \"sample\"\n}");
      }

      _ => panic!("This was supposed to be a plain http request"),
    }
  }

  #[test]
  fn test_parses_request_without_body() {
    match parse_request_http("GET https://example.com/comments\ncontent-type: application/json\n")
      .unwrap()
    {
      ParsedRequest::Http(request) => {
        assert_eq!(request.method, "GET");
        assert_eq!(request.url, "https://example.com/comments");
        assert_eq!(
          request.headers.get("content-type").unwrap(),
          "application/json"
        );
        assert!(request.body.is_empty());
      }
      _ => panic!("This was supposed to be a plain http request"),
    }
  }

  #[test]
  fn test_parses_request_without_headers_or_body() {
    match parse_request_http("DELETE https://example.com/comments\n").unwrap() {
      ParsedRequest::Http(request) => {
        assert_eq!(request.method, "DELETE");
        assert_eq!(request.url, "https://example.com/comments");
        assert!(request.headers.is_empty());
        assert!(request.body.is_empty());
      }

      _ => panic!("This was supposed to be a plain http request"),
    }
  }

  #[test]
  fn test_ignores_leading_blank_lines() {
    match parse_request_http("\n\nPUT https://example.com/comments\n").unwrap() {
      ParsedRequest::Http(request) => {
        assert_eq!(request.method, "PUT");
        assert_eq!(request.url, "https://example.com/comments");
      }

      _ => panic!("This was supposed to be a plain http request"),
    }
  }
}

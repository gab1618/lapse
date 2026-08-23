use crate::request::parsing::{MultipartRequestValue, ParsedRequest, UrlParser};

use super::parse_request_http;

#[test]
fn parses_multipart_req() {
  let raw_req = include_str!("../../../assets/with-multipart.md");
  let http_portion = raw_req.split_once("---").unwrap().0;
  let parsed = parse_request_http(http_portion, "https://").unwrap();
  match parsed {
    ParsedRequest::Multipart(req) => {
      assert_eq!(req.url, "https://example.com/comments");
      let found_content_type = req.headers.get("content-type").unwrap();
      assert_eq!(found_content_type, "multipart/form-data");

      let found_text_field = req.body.get("name").unwrap();
      let found_file_field = req.body.get("ex-file").unwrap();

      assert_eq!(
        found_text_field,
        &MultipartRequestValue::Text("${env.name}".to_owned())
      );
      assert_eq!(
        found_file_field,
        &MultipartRequestValue::File("./env.json".to_owned())
      );
    }
    _ => panic!("It was supposed to be a multipart request"),
  }
}

#[test]
fn parses_url_without_scheme() {
  let mut parser = UrlParser::new("localhost:3000", "http://");
  let parsed = parser.parse();
  assert_eq!(parsed, "http://localhost:3000");
}

#[test]
fn test_parses_sample_request() {
  let file_http = include_str!("../../../assets/request.md")
    .split_once("---")
    .unwrap()
    .0;

  match parse_request_http(file_http, "https://").unwrap() {
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
  match parse_request_http(
    "GET https://example.com/comments\ncontent-type: application/json\n",
    "https://",
  )
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
  match parse_request_http("DELETE https://example.com/comments\n", "https://").unwrap() {
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
  match parse_request_http("\n\nPUT https://example.com/comments\n", "https://").unwrap() {
    ParsedRequest::Http(request) => {
      assert_eq!(request.method, "PUT");
      assert_eq!(request.url, "https://example.com/comments");
    }

    _ => panic!("This was supposed to be a plain http request"),
  }
}

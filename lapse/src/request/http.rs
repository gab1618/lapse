use std::collections::HashMap;

use mlua::Lua;

use crate::{
  Lapse,
  env::hook::Event,
  log::ResponseLog,
  request::runner::{RequestRunner, Response},
};

impl Lapse {
  fn save_runner_log(&self, log: Response, name: String) -> crate::Result<ResponseLog> {
    let log = ResponseLog {
      request: name,
      text: log.text,
      status: log.status,
      headers: log.headers,
      duration: log.duration,
      timestamp: log.timestamp,
    };

    self.save_log(&log)?;

    Ok(log)
  }
  pub async fn request(&self, name: &str) -> crate::Result<ResponseLog> {
    self
      .request_with(name, self.get_runtime()?, self.get_hooks_scripts()?)
      .await
  }
  pub async fn request_with(
    &self,
    name: &str,
    runtime: Lua,
    hooks: HashMap<Event, Vec<String>>,
  ) -> crate::Result<ResponseLog> {
    let req = self.get_raw_request_http(name)?;
    let runner = RequestRunner::new(runtime, hooks);

    let response = runner.execute(&req).await?;

    let log = self.save_runner_log(response.clone(), name.to_string())?;

    Ok(log)
  }
}

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

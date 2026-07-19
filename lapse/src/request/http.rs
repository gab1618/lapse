use http::{Method, Request, Uri, Version};
use reqwest::Client;
use std::{collections::HashMap, str::FromStr};

use crate::{Lapse, log::ResponseLog, request::error::RequestError};

pub fn parse_request_http(doc: String) -> crate::Result<Request<Vec<u8>>> {
  let mut lines = doc.lines().skip_while(|line| line.is_empty());

  let request_line = lines.next().ok_or(RequestError::EmptyRequestFile)?;
  let mut request_parts = request_line.split_whitespace();
  let method = request_parts.next().ok_or(RequestError::MissingMethod)?;
  let uri = request_parts.next().ok_or(RequestError::MissingUri)?;

  let method = Method::from_str(method).map_err(RequestError::ParseMethod)?;
  let uri = Uri::from_str(uri).map_err(RequestError::ParseUri)?;

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

      let (name, value) = line.split_once(':').ok_or(RequestError::ParseHeaderLine)?;
      request_builder = request_builder.header(name.trim(), value.trim());
    } else {
      body_lines.push(line);
    }
  }

  Ok(
    request_builder
      .body(body_lines.join("\n").into_bytes())
      .map_err(RequestError::BuildRequest)?,
  )
}

impl Lapse {
  pub async fn request(&self, name: String) -> crate::Result<ResponseLog> {
    let client = Client::new();

    let req = self.get_request_file(&name)?;

    let request = self.resolve_request(&req)?;
    let parsed_request: reqwest::Request =
      request.try_into().map_err(RequestError::ConvertRequest)?;

    let response = client
      .execute(parsed_request)
      .await
      .map_err(RequestError::ExecuteRequest)?;

    let log_headers = response
      .headers()
      .iter()
      .map(|(name, value)| {
        let str_value = value
          .to_str()
          .map_err(RequestError::HeaderToStr)?
          .to_string();

        Ok((name.to_string(), str_value))
      })
      .collect::<crate::Result<HashMap<String, String>>>()?;

    let status_code = response.status().as_u16();
    let response_body = response
      .text()
      .await
      .map_err(RequestError::GetResponseBody)?;

    let log = ResponseLog {
      request: name,
      text: response_body,
      status: status_code,
      headers: log_headers,
    };

    self.save_log(&log)?;

    Ok(log)
  }
}

#[cfg(test)]
mod test {
  use http::Method;

  use crate::request::{RequestFile, http::parse_request_http};

  fn request_file(http: &str) -> RequestFile {
    RequestFile {
      markdown: None,
      http: http.to_owned(),
    }
  }

  #[test]
  fn test_parses_sample_request() {
    let file = request_file(
      include_str!("../../assets/request.md")
        .split_once("---")
        .unwrap()
        .0,
    );

    let request = parse_request_http(file.http).unwrap();

    assert_eq!(request.method(), Method::POST);
    assert_eq!(request.uri(), "https://example.com/comments");
    assert_eq!(
      request.headers().get("content-type").unwrap(),
      "application/json"
    );
    assert_eq!(
      std::str::from_utf8(request.body()).unwrap().trim(),
      "{\n  \"name\": \"sample\"\n}"
    );
  }

  #[test]
  fn test_parses_request_without_body() {
    let file = request_file("GET https://example.com/comments\ncontent-type: application/json\n");

    let request = parse_request_http(file.http).unwrap();

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

    let request = parse_request_http(file.http).unwrap();

    assert_eq!(request.method(), Method::DELETE);
    assert_eq!(request.uri(), "https://example.com/comments");
    assert!(request.headers().is_empty());
    assert!(request.body().is_empty());
  }

  #[test]
  fn test_ignores_leading_blank_lines() {
    let file = request_file("\n\nPUT https://example.com/comments\n");

    let request = parse_request_http(file.http).unwrap();

    assert_eq!(request.method(), Method::PUT);
    assert_eq!(request.uri(), "https://example.com/comments");
  }
}

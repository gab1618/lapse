use http::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use std::{collections::HashMap, str::FromStr as _};

use crate::{
  Lapse,
  eval::EvalCtx,
  log::ResponseLog,
  request::{
    error::RequestError,
    parsing::{MultipartRequestValue, ParsedRequest, parse_request_http},
  },
};

impl Lapse {
  pub async fn request(&self, name: String, ctx: EvalCtx) -> crate::Result<ResponseLog> {
    let client = Client::new();

    let req = self.get_request_http(&name)?;

    let solved_req = ctx.eval(&req)?;
    let request = parse_request_http(solved_req)?;
    let response = match request {
      ParsedRequest::Multipart(request) => {
        let mut form = reqwest::multipart::Form::new();

        for (field, value) in request.body {
          match value {
            MultipartRequestValue::File(f) => {
              form = form.file(field, f).await.unwrap();
            }
            MultipartRequestValue::Text(s) => {
              form = form.text(field, s);
            }
          }
        }
        let mut headers = HeaderMap::new();

        for (key, val) in request.headers {
          headers.insert(
            HeaderName::from_str(&key).unwrap(),
            HeaderValue::from_str(&val).unwrap(),
          );
        }
        client
          .post(request.url)
          .headers(headers)
          .multipart(form)
          .send()
          .await
          .map_err(RequestError::ExecuteRequest)?
      }
      ParsedRequest::Http(http_request) => {
        let parsed_request: reqwest::Request = http_request.try_into()?;

        client
          .execute(parsed_request)
          .await
          .map_err(RequestError::ExecuteRequest)?
      }
    };

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
  use crate::request::{
    RequestFile,
    parsing::{ParsedRequest, parse_request_http},
  };

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

    match parse_request_http(file.http).unwrap() {
      ParsedRequest::Multipart(_) => panic!("This was supposed to be a plain http request"),
      ParsedRequest::Http(request) => {
        assert_eq!(request.method, "POST");
        assert_eq!(request.url, "https://example.com/comments");
        assert_eq!(
          request.headers.get("content-type").unwrap(),
          "application/json"
        );
        assert_eq!(request.body.trim(), "{\n  \"name\": \"sample\"\n}");
      }
    }
  }

  #[test]
  fn test_parses_request_without_body() {
    let file = request_file("GET https://example.com/comments\ncontent-type: application/json\n");

    match parse_request_http(file.http).unwrap() {
      ParsedRequest::Multipart(_) => {
        panic!("This was supposed to be a plain http request")
      }
      ParsedRequest::Http(request) => {
        assert_eq!(request.method, "GET");
        assert_eq!(request.url, "https://example.com/comments");
        assert_eq!(
          request.headers.get("content-type").unwrap(),
          "application/json"
        );
        assert!(request.body.is_empty());
      }
    }
  }

  #[test]
  fn test_parses_request_without_headers_or_body() {
    let file = request_file("DELETE https://example.com/comments\n");

    match parse_request_http(file.http).unwrap() {
      ParsedRequest::Multipart(_) => {
        panic!("This was supposed to be a plain http request")
      }
      ParsedRequest::Http(request) => {
        assert_eq!(request.method, "DELETE");
        assert_eq!(request.url, "https://example.com/comments");
        assert!(request.headers.is_empty());
        assert!(request.body.is_empty());
      }
    }
  }

  #[test]
  fn test_ignores_leading_blank_lines() {
    let file = request_file("\n\nPUT https://example.com/comments\n");

    match parse_request_http(file.http).unwrap() {
      ParsedRequest::Multipart(_) => panic!("This was supposed to be a plan http request"),
      ParsedRequest::Http(request) => {
        assert_eq!(request.method, "PUT");
        assert_eq!(request.url, "https://example.com/comments");
      }
    }
  }
}

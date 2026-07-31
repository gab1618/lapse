use reqwest::Client;
use std::collections::HashMap;

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
    let request = parse_request_http(&solved_req)?;
    let response = match request {
      ParsedRequest::Multipart(request) => {
        let mut form = reqwest::multipart::Form::new();

        let headers = request.headers()?;

        for (field, value) in request.body {
          match value {
            MultipartRequestValue::File(f) => {
              form = form
                .file(field, f)
                .await
                .map_err(|_| RequestError::AddFile)?;
            }
            MultipartRequestValue::Text(s) => {
              form = form.text(field, s);
            }
          }
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
  use crate::request::parsing::{ParsedRequest, parse_request_http};

  #[test]
  fn test_parses_sample_request() {
    let file_http = include_str!("../../assets/request.md")
      .split_once("---")
      .unwrap()
      .0;

    match parse_request_http(file_http).unwrap() {
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
    match parse_request_http("GET https://example.com/comments\ncontent-type: application/json\n")
      .unwrap()
    {
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
    match parse_request_http("DELETE https://example.com/comments\n").unwrap() {
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
    match parse_request_http("\n\nPUT https://example.com/comments\n").unwrap() {
      ParsedRequest::Multipart(_) => panic!("This was supposed to be a plan http request"),
      ParsedRequest::Http(request) => {
        assert_eq!(request.method, "PUT");
        assert_eq!(request.url, "https://example.com/comments");
      }
    }
  }
}

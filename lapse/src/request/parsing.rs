use std::{collections::HashMap, str::FromStr};

use reqwest::{
  Body, Method, Request, Url,
  header::{HeaderName, HeaderValue},
};

use crate::request::error::RequestError;

pub struct HttpRequest {
  pub url: String,
  pub method: String,
  pub headers: HashMap<String, String>,
  pub body: String,
}

pub enum MultipartRequestValue {
  File(String),
  Text(String),
}
pub struct MultipartRequest {
  url: String,
  headers: HashMap<String, String>,
  body: HashMap<String, MultipartRequestValue>,
}

pub enum ParsedRequest {
  Http(HttpRequest),
  Multipart(MultipartRequest),
}

impl From<HttpRequest> for ParsedRequest {
  fn from(value: HttpRequest) -> Self {
    Self::Http(value)
  }
}
impl From<MultipartRequest> for ParsedRequest {
  fn from(value: MultipartRequest) -> Self {
    Self::Multipart(value)
  }
}

impl TryFrom<HttpRequest> for reqwest::Request {
  type Error = crate::Error;

  fn try_from(value: HttpRequest) -> Result<Self, Self::Error> {
    let mut req = Request::new(
      Method::from_str(&value.method).unwrap(),
      Url::from_str(&value.url).unwrap(),
    );
    let headers = req.headers_mut();

    for (name, value) in value.headers {
      headers.insert(
        HeaderName::from_str(&name).unwrap(),
        HeaderValue::from_str(&value).unwrap(),
      );
    }

    let body = req.body_mut();
    let parsed_body = Body::from(value.body);

    body.replace(parsed_body);
    Ok(req)
  }
}

pub fn parse_request_http(doc: String) -> crate::Result<ParsedRequest> {
  let mut lines = doc.lines().skip_while(|line| line.is_empty());

  let request_line = lines.next().ok_or(RequestError::EmptyRequestFile)?;
  let mut request_parts = request_line.split_whitespace();
  let method = request_parts.next().ok_or(RequestError::MissingMethod)?;
  let uri = request_parts.next().ok_or(RequestError::MissingUri)?;

  let mut headers = HashMap::new();

  // Parse headers
  for line in lines.by_ref() {
    if line.is_empty() {
      break;
    }
    let (name, value) = line.split_once(':').ok_or(RequestError::ParseHeaderLine)?;
    headers.insert(name.trim().to_owned(), value.trim().to_owned());
  }

  let raw_body = lines.collect::<Vec<&str>>().join("\n");

  match method {
    "MULTIPART" => Ok(
      MultipartRequest {
        url: uri.to_owned(),
        headers,
        body: parse_multipart_http_body(raw_body)?,
      }
      .into(),
    ),
    method => Ok(
      HttpRequest {
        url: uri.to_owned(),
        method: method.to_owned(),
        headers,
        body: raw_body,
      }
      .into(),
    ),
  }
}

fn parse_multipart_http_body(raw: String) -> crate::Result<HashMap<String, MultipartRequestValue>> {
  let lines = raw.lines();
  Ok(
    lines
      .map(|line| {
        let (name, value) = line.split_once(":").unwrap();
        let (name, value) = (name.trim().to_owned(), value.trim().to_owned());

        let parsed_value = parse_multipart_value(&value);
        (name.trim().to_owned(), parsed_value)
      })
      .collect::<HashMap<String, MultipartRequestValue>>(),
  )
}

fn parse_multipart_value(value: &str) -> MultipartRequestValue {
  todo!()
}

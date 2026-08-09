use std::{collections::HashMap, str::FromStr};

use http::HeaderMap;
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

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum MultipartRequestValue {
  File(String),
  Text(String),
}
pub struct MultipartRequest {
  pub url: String,
  pub headers: HashMap<String, String>,
  pub body: HashMap<String, MultipartRequestValue>,
}

pub struct GraphQLRequest {
  pub url: String,
  pub query: String,
  pub headers: HashMap<String, String>,
}

pub enum ParsedRequest {
  Http(HttpRequest),
  Multipart(MultipartRequest),
  GraphQL(GraphQLRequest),
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
impl From<GraphQLRequest> for ParsedRequest {
  fn from(value: GraphQLRequest) -> Self {
    Self::GraphQL(value)
  }
}

impl TryFrom<HttpRequest> for reqwest::Request {
  type Error = crate::Error;

  fn try_from(value: HttpRequest) -> Result<Self, Self::Error> {
    let mut req = Request::new(
      Method::from_str(&value.method).map_err(RequestError::ParseMethod)?,
      Url::from_str(&value.url).map_err(|_| RequestError::ParseUrl)?,
    );
    let headers = req.headers_mut();

    for (name, value) in value.headers {
      headers.insert(
        HeaderName::from_str(&name).map_err(|_| RequestError::ParseHeader)?,
        HeaderValue::from_str(&value).map_err(|_| RequestError::ParseHeader)?,
      );
    }

    let body = req.body_mut();
    let parsed_body = Body::from(value.body);

    body.replace(parsed_body);
    Ok(req)
  }
}

impl MultipartRequest {
  pub fn headers(&self) -> crate::Result<HeaderMap> {
    let headers = self
      .headers
      .iter()
      .map(|(key, value)| {
        Ok((
          HeaderName::from_str(key).map_err(|_| RequestError::ParseHeader)?,
          HeaderValue::from_str(value).map_err(|_| RequestError::ParseHeader)?,
        ))
      })
      .collect::<crate::Result<HeaderMap>>()?;

    Ok(headers)
  }
}

pub fn parse_request_http(doc: &str) -> crate::Result<ParsedRequest> {
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
    "GRAPHQL" => Ok(
      GraphQLRequest {
        url: uri.to_owned(),
        query: raw_body,
        headers,
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

  lines
    .map(|line| {
      let (name, value) = line
        .split_once(":")
        .ok_or(RequestError::EmptyMultipartValue)?;
      let (name, value) = (name.trim().to_owned(), value.trim().to_owned());

      let mut parser = MultipartValueParser::new(&value);
      let parsed_value = parser.parse()?;
      Ok((name.trim().to_owned(), parsed_value))
    })
    .collect::<crate::Result<HashMap<String, MultipartRequestValue>>>()
}

pub struct MultipartValueParser<'a> {
  src: &'a str,
  position: usize,
}

impl<'a> MultipartValueParser<'a> {
  pub fn new(src: &'a str) -> Self {
    Self { src, position: 0 }
  }
  pub fn parse(&mut self) -> crate::Result<MultipartRequestValue> {
    let next_char = self.peek().ok_or(RequestError::EmptyMultipartValue)?;

    match next_char {
      '"' => {
        self.bump();
        let s = self.consume_string();
        Ok(MultipartRequestValue::Text(s))
      }
      '@' => {
        self.bump();
        let p = self.consume_file();
        Ok(MultipartRequestValue::File(p))
      }
      c => Err(RequestError::InvalidMultipartCharacter(c).into()),
    }
  }
  fn bump(&mut self) {
    self.position += 1;
  }
  fn peek(&self) -> Option<char> {
    self.src[self.position..].chars().next()
  }
  fn consume_string(&mut self) -> String {
    let mut content = String::new();

    while let Some(c) = self.peek() {
      self.bump();
      if c == '"' {
        break;
      }

      content.push(c);
    }

    content
  }
  fn consume_file(&mut self) -> String {
    let mut content = String::new();

    while let Some(c) = self.peek() {
      self.bump();
      if c == '"' || c == ' ' {
        break;
      }

      content.push(c)
    }

    content
  }
}

#[cfg(test)]
mod test {
  use crate::request::parsing::{MultipartRequestValue, ParsedRequest};

  use super::parse_request_http;

  #[test]
  fn parses_multipart_req() {
    let raw_req = include_str!("../../assets/with-multipart.md");
    let http_portion = raw_req.split_once("---").unwrap().0;
    let parsed = parse_request_http(http_portion).unwrap();
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
}

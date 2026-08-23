use std::{collections::HashMap, str::FromStr};

use reqwest::{
  Body, Method, Request, Url,
  header::{HeaderName, HeaderValue},
};

use crate::request::{
  GraphQLRequest, HttpRequest, MultipartRequest, MultipartRequestValue, error::RequestError,
};

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

#[derive(serde::Serialize)]
struct GraphQLQueryBody {
  pub query: String,
  pub variables: HashMap<String, String>,
}
impl TryFrom<GraphQLRequest> for reqwest::Request {
  type Error = crate::Error;

  fn try_from(value: GraphQLRequest) -> Result<Self, Self::Error> {
    let mut req = Request::new(
      Method::POST,
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

    let query_body = GraphQLQueryBody {
      query: value.query,
      variables: Default::default(),
    };

    let body_query = serde_json::to_string(&query_body).unwrap();
    let parsed_body = Body::from(body_query);

    body.replace(parsed_body);
    Ok(req)
  }
}

pub fn parse_request_http(doc: &str, default_scheme: &str) -> crate::Result<ParsedRequest> {
  let mut lines = doc.lines().skip_while(|line| line.is_empty());

  let request_line = lines.next().ok_or(RequestError::EmptyRequestFile)?;
  let mut request_parts = request_line.split_whitespace();
  let method = request_parts.next().ok_or(RequestError::MissingMethod)?;
  let uri = request_parts.next().ok_or(RequestError::MissingUri)?;
  let mut url_parser = UrlParser::new(uri, default_scheme);
  let parsed_uri = url_parser.parse();

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
        url: parsed_uri,
        headers,
        body: parse_multipart_http_body(raw_body)?,
      }
      .into(),
    ),
    "GRAPHQL" => Ok(
      GraphQLRequest {
        url: parsed_uri,
        query: raw_body,
        headers,
      }
      .into(),
    ),
    method => Ok(
      HttpRequest {
        url: parsed_uri,
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

pub struct UrlParser<'a> {
  src: &'a str,
  pos: usize,
  default_scheme: &'a str,
}

impl<'a> UrlParser<'a> {
  pub fn new(src: &'a str, default_scheme: &'a str) -> Self {
    Self {
      src,
      pos: 0,
      default_scheme,
    }
  }
  fn peek_n(&self, n: usize) -> String {
    let elements = self.src[self.pos..].chars().take(n);
    elements.collect()
  }
  /// Consumes until delimiter.
  fn consume_until_delimiter(&mut self, delimiter: &str) -> Option<String> {
    let initial_pos = self.pos;

    loop {
      let next = self.peek_n(delimiter.len());
      self.bump_n(1);

      if next == delimiter {
        // Found delimiter, consuming string from initial pos to current pos
        let s: String = self.src[initial_pos..self.pos].chars().collect();
        return Some(s);
      }
      if next.is_empty() {
        break;
      }
    }

    // Delimiter not found. Resetting position and returning None
    self.pos = initial_pos;
    None
  }
  // Returns the default scheme if none was found
  fn consume_scheme(&mut self) -> String {
    self
      .consume_until_delimiter("://")
      .unwrap_or(String::from(self.default_scheme))
  }
  fn bump_n(&mut self, n: usize) {
    self.pos += n;
  }
  fn parse(&mut self) -> String {
    let scheme = self.consume_scheme();
    let remaining_chars: String = self.src[self.pos..].chars().collect();

    format!("{scheme}{remaining_chars}")
  }
}

#[cfg(test)]
mod test {
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
}

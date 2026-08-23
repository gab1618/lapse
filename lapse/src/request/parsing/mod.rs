use std::collections::HashMap;

use crate::request::{
  GraphQLRequest, HttpRequest, MultipartRequest, MultipartRequestValue, error::RequestError,
  parsing::url::UrlParser,
};

pub mod url;

#[cfg(test)]
mod test;

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

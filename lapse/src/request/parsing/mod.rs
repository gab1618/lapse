use std::collections::HashMap;

use crate::request::{
  GraphQLRequest, HttpRequest, MultipartRequest, MultipartRequestValue,
  error::RequestError,
  parsing::{inline_body::InlineBodyParamParser, url::UrlParser},
};

pub mod inline_body;
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

  let inline_params: Vec<_> = request_parts.collect();
  let parsed_inline_params = inline_params
    .into_iter()
    .map(|entry| {
      let mut parser = InlineBodyParamParser::new(entry);
      let (key, val) = parser.parse();

      (key, val)
    })
    .collect::<HashMap<_, _>>();

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
  let http_body = if parsed_inline_params.is_empty() {
    raw_body
  } else {
    serde_json::to_string(&parsed_inline_params).unwrap()
  };

  match method {
    "MULTIPART" => Ok(
      MultipartRequest {
        url: parsed_uri,
        headers,
        body: parse_multipart_http_body(http_body)?,
      }
      .into(),
    ),
    "GRAPHQL" => Ok(
      GraphQLRequest {
        url: parsed_uri,
        query: http_body,
        headers,
      }
      .into(),
    ),
    method => Ok(
      HttpRequest {
        url: parsed_uri,
        method: method.to_owned(),
        headers,
        body: http_body,
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

pub trait BaseParser<'a> {
  fn src(&self) -> &'a str;
  fn position(&self) -> usize;
  fn position_mut(&mut self) -> &mut usize;

  fn bump_n(&mut self, n: usize) {
    *self.position_mut() += n;
  }

  fn consume_until(&mut self, predicate: impl Fn(char) -> bool) -> Option<&'a str> {
    let start = self.position();

    loop {
      if let Some(c) = self.src()[self.position()..].chars().next() {
        if predicate(c) {
          return Some(&self.src()[start..self.position()]);
        }

        self.bump_n(c.len_utf8());
      } else {
        *self.position_mut() = start;
        return None;
      }
    }
  }
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

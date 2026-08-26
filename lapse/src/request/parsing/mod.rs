use std::collections::HashMap;

use crate::{
  request::{
    GraphQLRequest, HttpRequest, MultipartRequest, MultipartRequestValue,
    error::RequestError,
    parsing::{
      inline::body::{InlineParamParser, InlineItem, InlineItemKind},
      url::UrlParser,
    },
  },
  runner::value::Value,
};

pub mod inline;
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

  let inline_params = parse_inline_params(request_parts)?;

  let mut headers = HashMap::new();

  for line in lines.by_ref() {
    if line.is_empty() {
      break;
    }
    let (name, value) = line.split_once(':').ok_or(RequestError::ParseHeaderLine)?;
    headers.insert(name.trim().to_owned(), value.trim().to_owned());
  }

  if headers.is_empty() {
    headers.extend(
      inline_params
        .headers
        .into_iter()
        .map(|(key, value)| (key, value.to_string())),
    );
  }

  let url = if parsed_uri.contains('?') {
    parsed_uri
  } else {
    append_query_params(parsed_uri, inline_params.query)
  };

  let raw_body = lines.collect::<Vec<&str>>().join("\n");
  // Prioritize non-inline body
  let http_body = if !raw_body.trim().is_empty() || inline_params.body.is_empty() {
    raw_body
  } else {
    serde_json::to_string(&inline_params.body).map_err(RequestError::SerializeInlineBody)?
  };

  match method {
    "MULTIPART" => Ok(
      MultipartRequest {
        url,
        headers,
        body: parse_multipart_http_body(http_body)?,
      }
      .into(),
    ),
    "GRAPHQL" => Ok(
      GraphQLRequest {
        url,
        query: http_body,
        headers,
      }
      .into(),
    ),
    method => Ok(
      HttpRequest {
        url,
        method: method.to_owned(),
        headers,
        body: http_body,
      }
      .into(),
    ),
  }
}

struct InlineParams {
  query: HashMap<String, Value>,
  headers: HashMap<String, Value>,
  body: HashMap<String, Value>,
}

fn parse_inline_params<'a>(entries: impl Iterator<Item = &'a str>) -> crate::Result<InlineParams> {
  let mut query = HashMap::new();
  let mut headers = HashMap::new();
  let mut body = HashMap::new();

  for entry in entries {
    let InlineItem { kind, key, value } = InlineParamParser::new(entry).parse()?;

    match kind {
      InlineItemKind::Query => query.insert(key, value),
      InlineItemKind::Header => headers.insert(key, value),
      InlineItemKind::Body => body.insert(key, value),
    };
  }

  Ok(InlineParams {
    query,
    headers,
    body,
  })
}

fn append_query_params(url: String, params: HashMap<String, Value>) -> String {
  if params.is_empty() {
    return url;
  }

  let query_string = params
    .into_iter()
    .map(|(key, value)| format!("{key}={value}"))
    .collect::<Vec<_>>()
    .join("&");

  format!("{url}?{query_string}")
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
  fn consume_until(&mut self, predicate: impl Fn(char) -> bool) -> &'a str {
    let start = self.position();

    while let Some(c) = self.peek() {
      if predicate(c) {
        break;
      }
      *self.position_mut() += c.len_utf8();
    }

    &self.src()[start..self.position()]
  }
  fn peek(&self) -> Option<char> {
    self.src()[self.position()..].chars().next()
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

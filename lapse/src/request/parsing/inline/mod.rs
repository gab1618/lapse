use std::collections::HashMap;

use crate::{
  request::parsing::{BaseParser, inline::body::InlineParamParser, url::UrlParser},
  runner::value::Value,
};

pub mod body;

pub struct InlineRequestParser<'a> {
  src: &'a str,
  pos: usize,
  default_scheme: &'a str,
}

impl<'a> BaseParser<'a> for InlineRequestParser<'a> {
  fn src(&self) -> &'a str {
    self.src
  }

  fn position(&self) -> usize {
    self.pos
  }

  fn position_mut(&mut self) -> &mut usize {
    &mut self.pos
  }
}

#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Default)]
pub struct InlineRequest {
  pub method: String,
  pub uri: String,
  pub headers: HashMap<String, String>,
  pub body: HashMap<String, Value>,
  pub params: HashMap<String, String>,
}

impl<'a> InlineRequestParser<'a> {
  pub fn new(src: &'a str, default_scheme: &'a str) -> Self {
    Self {
      src,
      default_scheme,
      pos: 0,
    }
  }
  pub fn parse(&mut self) -> InlineRequest {
    let method = self.consume_until(char::is_whitespace);
    self.bump_n(1);
    let uri = self.consume_until(char::is_whitespace);

    let mut url_parser = UrlParser::new(uri, self.default_scheme);
    let parsed_uri = url_parser.parse();

    let mut result = InlineRequest {
      method: method.into(),
      uri: parsed_uri,
      ..Default::default()
    };

    let raw_params = &self.src[self.pos..];
    self.bump_n(raw_params.len());

    for entry in raw_params.split_whitespace() {
      let mut parser = InlineParamParser::new(entry);
      let parsed = parser.parse().unwrap();

      use body::InlineItemKind;

      match parsed.kind {
        InlineItemKind::Query => {
          result.params.insert(parsed.key, parsed.value.to_string());
        }
        InlineItemKind::Header => {
          result.headers.insert(parsed.key, parsed.value.to_string());
        }
        InlineItemKind::Body => {
          result.body.insert(parsed.key, parsed.value);
        }
      }
    }

    result
  }
}

#[cfg(test)]
mod test {
  use std::collections::HashMap;

  use super::{InlineRequest, InlineRequestParser};

  #[test]
  fn parses_simple_req() {
    let input = "GET example.com";
    let mut parser = InlineRequestParser::new(input, "http://");
    let parsed = parser.parse();

    assert_eq!(
      parsed,
      InlineRequest {
        method: "GET".to_string(),
        uri: "http://example.com".to_string(),
        ..Default::default()
      }
    )
  }
  #[test]
  fn parses_req_with_params() {
    let input = "POST example.com name==John";
    let mut parser = InlineRequestParser::new(input, "http://");
    let parsed = parser.parse();

    assert_eq!(
      parsed,
      InlineRequest {
        method: "POST".to_string(),
        uri: "http://example.com".to_string(),
        body: HashMap::from([("name".into(), "John".into())]),
        ..Default::default()
      }
    )
  }
}

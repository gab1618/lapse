use crate::request::parsing::{
  BaseParser,
  inline::body::{InlineItem, InlineParamParser},
  url::UrlParser,
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
pub struct InlineRequest {
  pub method: String,
  pub uri: String,
  pub params: Vec<InlineItem>,
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

    let raw_params = &self.src[self.pos..];
    self.bump_n(raw_params.len());
    let parsed_params = raw_params
      .split_whitespace()
      .map(|entry| {
        let mut parser = InlineParamParser::new(entry);
        parser.parse()
      })
      .collect::<crate::Result<Vec<_>>>()
      .unwrap();

    InlineRequest {
      method: method.to_string(),
      uri: parsed_uri,
      params: parsed_params,
    }
  }
}

#[cfg(test)]
mod test {
  use crate::request::parsing::inline::body::{InlineItem, InlineItemKind};

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
        params: Default::default(),
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
        params: vec![InlineItem {
          kind: InlineItemKind::Body,
          key: "name".to_string(),
          value: "John".into(),
        }],
      }
    )
  }
}

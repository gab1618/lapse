use crate::request::parsing::BaseParser;

pub mod body;

pub struct InlineRequestParser<'a> {
  src: &'a str,
  pos: usize,
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
}

impl<'a> InlineRequestParser<'a> {
  pub fn new(src: &'a str) -> Self {
    Self { src, pos: 0 }
  }
  pub fn parse(&mut self) -> InlineRequest {
    let method = self.consume_until(char::is_whitespace);
    self.bump_n(1);
    let uri = self.consume_until(char::is_whitespace);

    InlineRequest {
      method: method.to_string(),
      uri: uri.to_string(),
    }
  }
}

#[cfg(test)]
mod test {
  use super::{InlineRequest, InlineRequestParser};

  #[test]
  fn parses_simple_req() {
    let input = "GET example.com";
    let mut parser = InlineRequestParser::new(input);
    let parsed = parser.parse();

    assert_eq!(
      parsed,
      InlineRequest {
        method: "GET".to_string(),
        uri: "example.com".to_string()
      }
    )
  }
}

use crate::request::{
  error::RequestError,
  parsing::{BaseParser, url::UrlParser},
};

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
}

impl<'a> InlineRequestParser<'a> {
  pub fn new(src: &'a str, default_scheme: &'a str) -> Self {
    Self {
      src,
      default_scheme,
      pos: 0,
    }
  }
  pub fn parse(&mut self) -> crate::Result<InlineRequest> {
    let method = self.consume_until(char::is_whitespace);
    if method.is_empty() {
      return Err(RequestError::MissingMethod.into());
    }
    if self.peek().is_none() {
      return Err(RequestError::MissingUri.into());
    }
    self.bump_n(1);
    let uri = self.consume_until(char::is_whitespace);
    if uri.is_empty() {
      return Err(RequestError::MissingUri.into());
    }

    let mut url_parser = UrlParser::new(uri, self.default_scheme);
    let parsed_uri = url_parser.parse();

    let result = InlineRequest {
      method: method.into(),
      uri: parsed_uri,
    };

    let raw_params = &self.src[self.pos..];
    self.bump_n(raw_params.len());

    Ok(result)
  }
}

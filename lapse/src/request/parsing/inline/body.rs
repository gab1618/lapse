use crate::{request::error::RequestError, request::parsing::BaseParser, runner::value::Value};

pub struct InlineParamParser<'a> {
  src: &'a str,
  pos: usize,
}

impl<'a> BaseParser<'a> for InlineParamParser<'a> {
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

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum InlineItemKind {
  Query,
  Header,
  Body,
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct InlineItem {
  pub kind: InlineItemKind,
  pub key: String,
  pub value: Value,
}

impl<'a> InlineParamParser<'a> {
  pub fn new(src: &'a str) -> Self {
    Self { src, pos: 0 }
  }
  pub fn parse(&mut self) -> crate::Result<InlineItem> {
    let param_name = self
      .consume_until(|c| c == '=' || c == ':' || c == '?')
      .to_string();

    if param_name.is_empty() {
      return Err(RequestError::ParseInlineParam.into());
    }

    let kind_char = self.peek().ok_or(RequestError::ParseInlineParam)?;
    self.bump_n(1);

    let kind = match kind_char {
      '=' => InlineItemKind::Body,
      '?' => InlineItemKind::Query,
      ':' => InlineItemKind::Header,
      _ => unreachable!("consume_until only stops at '=', ':' or '?'"),
    };

    let parsed_value = if kind_char == ':' {
      Value::String(self.src[self.pos..].to_string())
    } else {
      match self.src[self.pos..].chars().next() {
        Some(':') => {
          self.bump_n(1);
          serde_json::from_str(&self.src[self.pos..]).map_err(RequestError::ParseInlineJsonValue)?
        }
        Some('=') => {
          self.bump_n(1);
          Value::String(self.src[self.pos..].to_string())
        }
        _ => Value::String(self.src[self.pos..].to_string()),
      }
    };

    Ok(InlineItem {
      kind,
      key: param_name,
      value: parsed_value,
    })
  }
}

#[cfg(test)]
mod test {
  use super::{InlineItemKind, InlineParamParser};

  #[test]
  fn test_parses_inline_body_param() {
    let src = "name==John";
    let mut parser = InlineParamParser::new(src);
    let parsed = parser.parse().unwrap();
    assert_eq!(parsed.key, "name");
    assert_eq!(parsed.value, "John".into());
    assert_eq!(parsed.kind, InlineItemKind::Body);
  }
  #[test]
  fn test_parses_body_number() {
    let src = "age=:32";
    let mut parser = InlineParamParser::new(src);
    let parsed = parser.parse().unwrap();

    assert_eq!(parsed.key, "age");
    assert_eq!(parsed.value, 32.into());
    assert_eq!(parsed.kind, InlineItemKind::Body);
  }
  #[test]
  fn test_parses_query_param() {
    let src = "id?=abc";
    let mut parser = InlineParamParser::new(src);
    let parsed = parser.parse().unwrap();

    assert_eq!(parsed.key, "id");
    assert_eq!(parsed.value, "abc".into());
    assert_eq!(parsed.kind, InlineItemKind::Query);
  }

  #[test]
  fn test_parses_query_param_number() {
    let src = "page?:1";
    let mut parser = InlineParamParser::new(src);
    let parsed = parser.parse().unwrap();

    assert_eq!(parsed.key, "page");
    assert_eq!(parsed.value, 1.into());
    assert_eq!(parsed.kind, InlineItemKind::Query);
  }

  #[test]
  fn test_errors_on_missing_separator() {
    let src = "name";
    let mut parser = InlineParamParser::new(src);
    assert!(parser.parse().is_err());
  }

  #[test]
  fn test_errors_on_invalid_json_value() {
    let src = "age=:not_json";
    let mut parser = InlineParamParser::new(src);
    assert!(parser.parse().is_err());
  }

  #[test]
  fn test_errors_on_missing_key() {
    let src = "=John";
    let mut parser = InlineParamParser::new(src);
    assert!(parser.parse().is_err());
  }
}

use crate::{
  request::{MultipartRequestValue, error::RequestError, parsing::BaseParser},
  runner::value::Value,
};

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
  Form,
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum InlineValue {
  Value(Value),
  Form(MultipartRequestValue),
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct InlineItem {
  pub kind: InlineItemKind,
  pub key: String,
  pub value: InlineValue,
}

impl<'a> InlineParamParser<'a> {
  pub fn new(src: &'a str) -> Self {
    Self { src, pos: 0 }
  }
  pub fn parse(&mut self) -> crate::Result<InlineItem> {
    let param_name = self
      .consume_until(|c| matches!(c, '=' | ':' | '?' | '@'))
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
      '@' => InlineItemKind::Form,
      _ => unreachable!("consume_until only stops at '=', ':', '?' or '@'"),
    };

    let value = match kind_char {
      ':' => InlineValue::Value(Value::String(self.src[self.pos..].to_string())),
      // `@` introduces a form field: a further `@` marks a file, `=` (or nothing) marks plain text
      '@' => InlineValue::Form(match self.src[self.pos..].chars().next() {
        Some('@') => {
          self.bump_n(1);
          MultipartRequestValue::File(self.src[self.pos..].to_string())
        }
        Some('=') => {
          self.bump_n(1);
          MultipartRequestValue::Text(self.src[self.pos..].to_string())
        }
        _ => MultipartRequestValue::Text(self.src[self.pos..].to_string()),
      }),
      _ => InlineValue::Value(match self.src[self.pos..].chars().next() {
        Some(':') => {
          self.bump_n(1);
          serde_json::from_str(&self.src[self.pos..]).map_err(RequestError::ParseInlineJsonValue)?
        }
        Some('=') => {
          self.bump_n(1);
          Value::String(self.src[self.pos..].to_string())
        }
        _ => Value::String(self.src[self.pos..].to_string()),
      }),
    };

    Ok(InlineItem {
      kind,
      key: param_name,
      value,
    })
  }
}

#[cfg(test)]
mod test {
  use super::{InlineItemKind, InlineParamParser, InlineValue};
  use crate::request::MultipartRequestValue;

  #[test]
  fn test_parses_inline_body_param() {
    let src = "name==John";
    let mut parser = InlineParamParser::new(src);
    let parsed = parser.parse().unwrap();
    assert_eq!(parsed.key, "name");
    assert_eq!(parsed.value, InlineValue::Value("John".into()));
    assert_eq!(parsed.kind, InlineItemKind::Body);
  }
  #[test]
  fn test_parses_body_number() {
    let src = "age=:32";
    let mut parser = InlineParamParser::new(src);
    let parsed = parser.parse().unwrap();

    assert_eq!(parsed.key, "age");
    assert_eq!(parsed.value, InlineValue::Value(32.into()));
    assert_eq!(parsed.kind, InlineItemKind::Body);
  }
  #[test]
  fn test_parses_query_param() {
    let src = "id?=abc";
    let mut parser = InlineParamParser::new(src);
    let parsed = parser.parse().unwrap();

    assert_eq!(parsed.key, "id");
    assert_eq!(parsed.value, InlineValue::Value("abc".into()));
    assert_eq!(parsed.kind, InlineItemKind::Query);
  }

  #[test]
  fn test_parses_query_param_number() {
    let src = "page?:1";
    let mut parser = InlineParamParser::new(src);
    let parsed = parser.parse().unwrap();

    assert_eq!(parsed.key, "page");
    assert_eq!(parsed.value, InlineValue::Value(1.into()));
    assert_eq!(parsed.kind, InlineItemKind::Query);
  }

  #[test]
  fn test_parses_form_text_field() {
    let src = "name@=John";
    let mut parser = InlineParamParser::new(src);
    let parsed = parser.parse().unwrap();

    assert_eq!(parsed.key, "name");
    assert_eq!(
      parsed.value,
      InlineValue::Form(MultipartRequestValue::Text("John".to_owned()))
    );
    assert_eq!(parsed.kind, InlineItemKind::Form);
  }

  #[test]
  fn test_parses_form_text_field_without_suffix() {
    let src = "name@John";
    let mut parser = InlineParamParser::new(src);
    let parsed = parser.parse().unwrap();

    assert_eq!(
      parsed.value,
      InlineValue::Form(MultipartRequestValue::Text("John".to_owned()))
    );
  }

  #[test]
  fn test_parses_form_file_field() {
    let src = "avatar@@./photo.png";
    let mut parser = InlineParamParser::new(src);
    let parsed = parser.parse().unwrap();

    assert_eq!(parsed.key, "avatar");
    assert_eq!(
      parsed.value,
      InlineValue::Form(MultipartRequestValue::File("./photo.png".to_owned()))
    );
    assert_eq!(parsed.kind, InlineItemKind::Form);
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

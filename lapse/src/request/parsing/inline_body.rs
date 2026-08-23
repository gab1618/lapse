use crate::{request::parsing::BaseParser, runner::value::Value};

pub struct InlineBodyParamParser<'a> {
  src: &'a str,
  pos: usize,
}

impl<'a> BaseParser<'a> for InlineBodyParamParser<'a> {
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

impl<'a> InlineBodyParamParser<'a> {
  pub fn new(src: &'a str) -> Self {
    Self { src, pos: 0 }
  }
  pub fn parse(&mut self) -> (String, Value) {
    let param_name = self
      .consume_until(|c| c == '=' || c == ':')
      .unwrap()
      .to_string();
    let separator = self.src[self.pos..].chars().next().unwrap();
    self.bump_n(1);
    let raw_value: String = self.src[self.pos..].chars().collect();

    let parsed_value = match separator {
      ':' => {
        let num: f64 = raw_value.parse().unwrap();
        Value::Number(num)
      }
      '=' => Value::String(raw_value),
      _ => todo!(),
    };

    (param_name, parsed_value)
  }
}

#[cfg(test)]
mod test {
  use crate::request::parsing::inline_body::InlineBodyParamParser;

  #[test]
  fn test_parses_inline_body_param() {
    let src = "name=John";
    let mut parser = InlineBodyParamParser::new(src);
    let (key, val) = parser.parse();
    assert_eq!(key, "name");
    assert_eq!(val, "John".into());
  }
  #[test]
  fn test_parses_inline_number() {
    let src = "age:32";
    let mut parser = InlineBodyParamParser::new(src);
    let (key, val) = parser.parse();

    assert_eq!(key, "age");
    assert_eq!(val, 32.0.into());
  }
}

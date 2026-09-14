use crate::runner::{Runner, value::Value};
use chumsky::prelude::*;

#[derive(PartialEq, Debug)]
pub enum DocumentToken {
  String(String),
  Expr(String),
}

pub fn interpolated_parser<'a>() -> impl Parser<'a, &'a str, String> {
  just("${")
    .ignore_then(recursive(|this| {
      let text = none_of("{}").repeated().at_least(1).collect::<String>();

      let nested = just('{')
        .ignore_then(this.clone())
        .then_ignore(just('}'))
        .map(|inner| format!("{{{inner}}}"));

      choice((text, nested)).repeated().collect::<String>()
    }))
    .then_ignore(just('}'))
}

pub fn document_parser<'a>() -> impl Parser<'a, &'a str, Vec<DocumentToken>> {
  let interpolated = interpolated_parser().map(DocumentToken::Expr);

  let literal = none_of("$")
    .repeated()
    .at_least(1)
    .collect::<String>()
    .map(DocumentToken::String);

  choice((interpolated, literal)).repeated().collect()
}

impl Runner {
  pub fn eval(&self, doc: &str) -> crate::Result<String> {
    let parser = document_parser();
    let tokens = parser.parse(doc).unwrap();
    let mut result = String::new();

    for token in tokens {
      match token {
        DocumentToken::String(inner) => {
          result.push_str(&inner);
        }
        DocumentToken::Expr(inner) => {
          let value: Value = self.runtime.load(inner).eval()?;
          result.push_str(&value.to_string());
        }
      }
    }

    Ok(result)
  }
}

#[cfg(test)]
mod test {
  use chumsky::Parser;

  use crate::runner::eval::{document_parser, interpolated_parser};

  use super::DocumentToken;

  #[test]
  fn test_parses_interpolated() {
    let parser = interpolated_parser();

    let parsed = parser.parse("${Env.name}").unwrap();

    assert_eq!(parsed, "Env.name");
  }

  #[test]
  fn test_parses_document() {
    let parser = document_parser();

    let parsed = parser.parse("name: ${name}").unwrap();
    assert_eq!(parsed[0], DocumentToken::String("name: ".to_owned()));
    assert_eq!(parsed[1], DocumentToken::Expr("name".to_owned()));
  }
}

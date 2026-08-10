#[derive(PartialEq, Debug)]
pub enum DocumentToken {
  String(String),
  Expr(String),
}

pub struct DocumentLexer<'a> {
  src: &'a str,
  pos: usize,
}

impl<'a> DocumentLexer<'a> {
  pub fn new(src: &'a str) -> Self {
    Self { src, pos: 0 }
  }

  pub fn tokenize(&mut self) -> Vec<DocumentToken> {
    let mut tokens = vec![];

    loop {
      let string = self.consume_string();
      if !string.is_empty() {
        tokens.push(DocumentToken::String(string));
      }

      if self.peek().is_none() {
        break;
      }

      tokens.push(DocumentToken::Expr(self.consume_expr()));
    }

    tokens
  }

  // assumes the current position is at the `$` of a `${ ... }` expression.
  // a `\` escapes the next char, taking it out of depth-tracking, so a
  // literal `{` or `}` inside the expression (e.g. a Lua table constructor)
  // can be written as `\{` / `\}` without confusing the matching `}` search
  fn consume_expr(&mut self) -> String {
    self.bump();
    self.bump();

    let mut content = String::new();
    let mut depth = 1;

    while let Some(c) = self.peek() {
      match c {
        '\\' => {
          self.bump();
          if let Some(escaped) = self.peek() {
            content.push(escaped);
            self.bump();
          }
        }
        '{' => {
          depth += 1;
          content.push(c);
          self.bump();
        }
        '}' => {
          depth -= 1;
          self.bump();
          if depth == 0 {
            break;
          }
          content.push(c);
        }
        _ => {
          content.push(c);
          self.bump();
        }
      }
    }

    content
  }

  fn consume_string(&mut self) -> String {
    let mut content = String::new();

    while let Some(c) = self.peek() {
      if c == '$' && self.peek_at(1) == Some('{') {
        break;
      }

      content.push(c);
      self.bump();
    }

    content
  }

  fn peek(&self) -> Option<char> {
    self.src[self.pos..].chars().next()
  }

  fn peek_at(&self, n: usize) -> Option<char> {
    self.src[self.pos..].chars().nth(n)
  }

  fn bump(&mut self) -> Option<char> {
    let c = self.peek()?;
    self.pos += c.len_utf8();
    Some(c)
  }
}

#[cfg(test)]
mod test {
  use super::{DocumentLexer, DocumentToken};

  fn tokenize(src: &str) -> Vec<DocumentToken> {
    DocumentLexer::new(src).tokenize()
  }

  #[test]
  fn test_tokenizes_plain_string() {
    let tokens = tokenize("{\n  \"name\": \"sample\"\n}");

    assert_eq!(
      tokens,
      vec![DocumentToken::String(
        "{\n  \"name\": \"sample\"\n}".to_owned()
      )]
    );
  }

  #[test]
  fn test_tokenizes_expr_interpolation() {
    let tokens = tokenize("{\n  \"name\": ${var(\"name\")}\n}");

    assert_eq!(
      tokens,
      vec![
        DocumentToken::String("{\n  \"name\": ".to_owned()),
        DocumentToken::Expr("var(\"name\")".to_owned()),
        DocumentToken::String("\n}".to_owned()),
      ]
    );
  }

  #[test]
  fn test_tokenizes_sample_asset() {
    let http = include_str!("../../assets/with-expr.md")
      .split_once("---")
      .unwrap()
      .0
      .trim_start_matches('\n');

    let tokens = tokenize(http);

    assert_eq!(
      tokens,
      vec![
        DocumentToken::String(
          "POST https://example.com/comments\ncontent-type: application/json\n\n{\n  \"name\": \""
            .to_owned()
        ),
        DocumentToken::Expr("env.name".to_owned()),
        DocumentToken::String("\"\n}\n\n".to_owned()),
      ]
    );
  }

  #[test]
  fn test_expr_with_nested_parens() {
    let tokens = tokenize("${fn(\"a\", nested(1))}");

    assert_eq!(
      tokens,
      vec![DocumentToken::Expr("fn(\"a\", nested(1))".to_owned())]
    );
  }

  #[test]
  fn test_expr_with_nested_braces() {
    let tokens = tokenize("${fn({a = 1})}");

    assert_eq!(tokens, vec![DocumentToken::Expr("fn({a = 1})".to_owned())]);
  }

  #[test]
  fn test_expr_with_escaped_braces() {
    let tokens = tokenize("${fn(\"\\{literal\\}\")}");

    assert_eq!(
      tokens,
      vec![DocumentToken::Expr("fn(\"{literal}\")".to_owned())]
    );
  }

  #[test]
  fn test_leaves_lone_dollar_sign_as_string() {
    let tokens = tokenize("price: $5");

    assert_eq!(tokens, vec![DocumentToken::String("price: $5".to_owned())]);
  }
}

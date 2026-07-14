pub enum RequestToken {
  String(String),
  Expr(String),
}

pub struct RequestTokenizer<'a> {
  src: &'a str,
  pos: usize,
}

impl<'a> RequestTokenizer<'a> {
  pub fn tokenize(&self) -> Vec<RequestToken> {
    todo!()
  }
  fn consume_expr(&mut self) -> String {

    String::new()
  }
  fn consume_string(&mut self) -> String {
    let mut content = String::new();

    while self.peek() != Some('$') {
      match self.next() {
        Some(c) => {
          content.push(c);
        }
        None => (),
      }
    }

    content
  }
  fn peek(&mut self) -> Option<char> {
    self.src[self.pos..].chars().next()
  }
  fn next(&mut self) -> Option<char> {
    self.pos += 1;
    self.src[self.pos..].chars().next()
  }
}

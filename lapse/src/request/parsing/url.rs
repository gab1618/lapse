pub struct UrlParser<'a> {
  src: &'a str,
  pos: usize,
  default_scheme: &'a str,
}

impl<'a> UrlParser<'a> {
  pub fn new(src: &'a str, default_scheme: &'a str) -> Self {
    Self {
      src,
      pos: 0,
      default_scheme,
    }
  }
  fn peek_n(&self, n: usize) -> String {
    let elements = self.src[self.pos..].chars().take(n);
    elements.collect()
  }
  /// Consumes until delimiter.
  fn consume_until_delimiter(&mut self, delimiter: &str) -> Option<String> {
    let initial_pos = self.pos;

    loop {
      let next = self.peek_n(delimiter.len());
      self.bump_n(1);

      if next == delimiter {
        // Found delimiter, consuming string from initial pos to current pos
        let s: String = self.src[initial_pos..self.pos].chars().collect();
        return Some(s);
      }
      if next.is_empty() {
        break;
      }
    }

    // Delimiter not found. Resetting position and returning None
    self.pos = initial_pos;
    None
  }
  // Returns the default scheme if none was found
  fn consume_scheme(&mut self) -> String {
    self
      .consume_until_delimiter("://")
      .unwrap_or(String::from(self.default_scheme))
  }
  fn bump_n(&mut self, n: usize) {
    self.pos += n;
  }
  pub fn parse(&mut self) -> String {
    let scheme = self.consume_scheme();
    let remaining_chars: String = self.src[self.pos..].chars().collect();

    format!("{scheme}{remaining_chars}")
  }
}

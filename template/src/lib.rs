pub mod templates;
use std::fmt::Display;

pub struct RequestFile {
  markdown: String,
  http: String,
}

impl Display for RequestFile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{}", self.http)?;
    writeln!(f, "---")?;
    writeln!(f, "{}", self.markdown)
  }
}

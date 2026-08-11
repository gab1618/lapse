pub mod error;
pub mod openapi;
pub mod templates;

use std::{fmt::Display, path::Path};

pub use error::{Error, Result};

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

pub trait Generator {
  fn load<P: AsRef<Path>>(&self, path: P) -> Result<()>;
}

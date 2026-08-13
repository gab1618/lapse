use std::fmt::Display;

pub struct RequestFile {
  pub method: String,
  pub url: String,
  pub title: String,
  pub description: String,
}

impl Display for RequestFile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // Request section
    writeln!(f, "{} {}", self.method, self.url)?;
    writeln!(f)?;

    // Markdown section
    writeln!(f, "---")?;
    writeln!(f, "# {}", self.title)?;
    writeln!(f, "{}", self.description)?;

    Ok(())
  }
}

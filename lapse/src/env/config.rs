use std::fmt::Display;

#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(serde::Deserialize)]
pub enum DefaultSchema {
  #[serde(rename = "https")]
  Https,
  #[serde(rename = "http")]
  Http,
}

impl Default for DefaultSchema {
  fn default() -> Self {
    Self::Http
  }
}

impl Display for DefaultSchema {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use DefaultSchema::{Http, Https};

    match self {
      Https => write!(f, "https://")?,
      Http => write!(f, "http://")?,
    }

    Ok(())
  }
}

#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(serde::Deserialize, Default)]
pub struct EnvConfig {
  pub default_schema: DefaultSchema,
}

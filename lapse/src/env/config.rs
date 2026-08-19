use std::fmt::Display;

#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(serde::Deserialize)]
pub enum DefaultScheme {
  #[serde(rename = "https")]
  Https,
  #[serde(rename = "http")]
  Http,
}

impl Default for DefaultScheme {
  fn default() -> Self {
    Self::Http
  }
}

impl Display for DefaultScheme {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use DefaultScheme::{Http, Https};

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
  pub default_scheme: DefaultScheme,
}

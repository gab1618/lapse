use std::collections::HashMap;

use crate::templates::TemplateCollection;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenApi {
  pub openapi: String,
  pub info: Info,
  pub paths: HashMap<String, PathItem>,
  pub servers: Option<Vec<Server>>,
}

#[derive(Debug, Deserialize)]
pub struct Info {
  pub title: String,
  pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
  pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct PathItem {
  pub get: Option<Operation>,
  pub post: Option<Operation>,
  pub put: Option<Operation>,
  pub patch: Option<Operation>,
  pub delete: Option<Operation>,
  pub head: Option<Operation>,
  pub options: Option<Operation>,
  pub trace: Option<Operation>,
}

#[derive(Debug, Deserialize)]
pub struct Operation {
  pub summary: Option<String>,
  pub description: Option<String>,
}
impl From<OpenApi> for TemplateCollection {
  fn from(_value: OpenApi) -> Self {
    vec![].into()
  }
}

#[cfg(test)]
mod test {
  use crate::openapi::OpenApi;

  #[test]
  fn test_parse_example() {
    let parsed: OpenApi =
      serde_yaml::from_str(include_str!("../../assets/ex-schema.yaml")).unwrap();
    assert_eq!(parsed.info.title, "Example API");
  }
}

use std::collections::HashMap;

use crate::templates::TemplateCollection;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenApi {
  pub openapi: String,
  pub info: Info,
  pub paths: HashMap<String, HashMap<PathMethod, Operation>>,
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

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum PathMethod {
  #[serde(rename = "get")]
  Get,
  #[serde(rename = "post")]
  Post,
  #[serde(rename = "patch")]
  Patch,
  #[serde(rename = "delete")]
  Delete,
  #[serde(rename = "put")]
  Put,
  #[serde(rename = "head")]
  Head,
  #[serde(rename = "options")]
  Options,
  #[serde(rename = "trace")]
  Trace,
}

#[derive(Debug, Deserialize)]
pub struct Operation {
  pub summary: Option<String>,
  pub description: Option<String>,
}
impl From<OpenApi> for TemplateCollection {
  fn from(value: OpenApi) -> Self {
    let servers = value.servers.unwrap();
    let main_server = servers.get(0).unwrap();
    let base_url = &main_server.url;

    for (path, details) in value.paths {
      let full_url = format!("{}{}", base_url, path);

      for (method, details) in details {
        todo!()
      }
    }
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

use std::collections::HashMap;

use crate::templates::{
  LapsePreset, TemplateEntry, openapi::error::OpenApiError, request_file::RequestFile,
};

use serde::Deserialize;

pub mod error;

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

impl ToString for PathMethod {
  fn to_string(&self) -> String {
    match self {
      PathMethod::Get => "GET",
      PathMethod::Post => "POST",
      PathMethod::Patch => "PATCH",
      PathMethod::Delete => "Delete",
      PathMethod::Put => "PUT",
      PathMethod::Head => "HEAD",
      PathMethod::Options => "OPTIONS",
      PathMethod::Trace => "TRACE",
    }
    .to_string()
  }
}

#[derive(Debug, Deserialize)]
pub struct Operation {
  pub summary: Option<String>,
  pub description: Option<String>,
}

impl TryFrom<OpenApi> for LapsePreset {
  type Error = crate::Error;

  fn try_from(value: OpenApi) -> Result<Self, Self::Error> {
    let servers = value.servers.ok_or(OpenApiError::NoServerAvailable)?;
    let main_server = servers.first().ok_or(OpenApiError::NoServerAvailable)?;
    let base_url = &main_server.url;

    let mut request_files = vec![];

    for (path, details) in value.paths {
      let full_url = format!("{}{}", base_url, path);

      for (method, details) in details {
        let request_file = RequestFile {
          method: method.to_string(),
          url: full_url.clone(),
          title: details.summary.unwrap_or_default(),
          description: details.description.unwrap_or_default(),
        };
        let request_file_content = format!("{}", request_file);
        let file_name = format!("{}.md", method.to_string().to_lowercase());

        request_files.push(
          TemplateEntry {
            name: file_name,
            content: request_file_content,
          }
          .into(),
        );
      }
    }

    Ok(LapsePreset {
      requests: request_files.into(),
      ..Default::default()
    })
  }
}

impl OpenApi {
  pub fn from_str_schema(schema: &str) -> crate::Result<Self> {
    Ok(serde_yaml::from_str(schema).map_err(OpenApiError::ParseSchema)?)
  }
}

#[cfg(test)]
mod test {
  use tempfile::tempdir;

  use crate::templates::LapsePreset;

  use super::OpenApi;

  #[test]
  fn test_parse_example() {
    let parsed: OpenApi =
      serde_yaml::from_str(include_str!("../../../assets/ex-schema.yaml")).unwrap();
    assert_eq!(parsed.info.title, "Example API");
  }

  #[test]
  fn test_load_preset() {
    let parsed: OpenApi =
      serde_yaml::from_str(include_str!("../../../assets/ex-schema.yaml")).unwrap();
    let preset: LapsePreset = parsed.try_into().unwrap();
    let dir = tempdir().unwrap();
    preset.load(dir.path()).unwrap();
  }
}

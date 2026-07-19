#[derive(Debug, thiserror::Error)]
pub enum RequestError {
  #[error("Could not read request file: {0}")]
  ReadRequestFile(#[source] std::io::Error),
  #[error("Could not read collection dir: {0}")]
  ReadCollectionDir(#[source] std::io::Error),
  #[error("Empty request file")]
  EmptyRequestFile,
  #[error("Missing method")]
  MissingMethod,
  #[error("Missing URI")]
  MissingUri,
  #[error("Could not parse method: {0}")]
  ParseMethod(#[source] http::method::InvalidMethod),
  #[error("Could not parse URI: {0}")]
  ParseUri(#[source] http::uri::InvalidUri),
  #[error("Could not parse header line")]
  ParseHeaderLine,
  #[error("Could not build request: {0}")]
  BuildRequest(#[source] http::Error),
  #[error("Could not convert into request: {0}")]
  ConvertRequest(#[source] reqwest::Error),
  #[error("Could not execute request: {0}")]
  ExecuteRequest(#[source] reqwest::Error),
  #[error("Could not get response body: {0}")]
  GetResponseBody(#[source] reqwest::Error),
  #[error("Could not convert header into string: {0}")]
  HeaderToStr(#[source] http::header::ToStrError),
}

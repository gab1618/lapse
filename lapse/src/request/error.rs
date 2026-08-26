#[derive(Debug, thiserror::Error)]
pub enum RequestError {
  #[error("Could not read request file: {0}")]
  ReadRequestFile(#[source] std::io::Error),
  #[error("Could not read collection dir: {0}")]
  ReadCollectionDir(#[source] std::io::Error),
  #[error("Could not parse collection path")]
  ParseCollectionPath,
  #[error("Empty request file")]
  EmptyRequestFile,
  #[error("Missing method")]
  MissingMethod,
  #[error("Missing URI")]
  MissingUri,
  #[error("Could not parse method: {0}")]
  ParseMethod(#[source] http::method::InvalidMethod),
  #[error("Could not parse Url")]
  ParseUrl,
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
  #[error("Could not resolve http line")]
  ResolveHttpLine,
  #[error("Invalid character: {0}")]
  InvalidMultipartCharacter(char),
  #[error("Could not parse header")]
  ParseHeader,
  #[error("Could not add file")]
  AddFile,
  #[error("Could not parse multipart form value: empty")]
  EmptyMultipartValue,
  #[error("Could not parse inline param: missing '=', ':' or '?'")]
  ParseInlineParam,
  #[error("Could not parse inline JSON value: {0}")]
  ParseInlineJsonValue(#[source] serde_json::Error),
  #[error("Could not serialize inline body params: {0}")]
  SerializeInlineBody(#[source] serde_json::Error),
  #[error("Could not create request client: {0}")]
  CreateClient(#[source] reqwest::Error),
}

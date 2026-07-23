use std::str::FromStr as _;

use http::{Method, Request, Uri, Version};

use crate::request::error::RequestError;

pub fn parse_request_http(doc: String) -> crate::Result<Request<Vec<u8>>> {
  let mut lines = doc.lines().skip_while(|line| line.is_empty());

  let request_line = lines.next().ok_or(RequestError::EmptyRequestFile)?;
  let mut request_parts = request_line.split_whitespace();
  let method = request_parts.next().ok_or(RequestError::MissingMethod)?;
  let uri = request_parts.next().ok_or(RequestError::MissingUri)?;

  let method = Method::from_str(method).map_err(RequestError::ParseMethod)?;
  let uri = Uri::from_str(uri).map_err(RequestError::ParseUri)?;

  let mut request_builder = Request::builder()
    .method(method)
    .uri(uri)
    .version(Version::HTTP_11);

  let mut body_started = false;
  let mut body_lines = vec![];

  for line in lines {
    if !body_started {
      if line.is_empty() {
        body_started = true;
        continue;
      }

      let (name, value) = line.split_once(':').ok_or(RequestError::ParseHeaderLine)?;
      request_builder = request_builder.header(name.trim(), value.trim());
    } else {
      body_lines.push(line);
    }
  }

  Ok(
    request_builder
      .body(body_lines.join("\n").into_bytes())
      .map_err(RequestError::BuildRequest)?,
  )
}

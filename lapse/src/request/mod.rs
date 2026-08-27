use ::http::{HeaderMap, HeaderName, HeaderValue, Method};
use reqwest::{Body, Request, Url};

use crate::{Lapse, request::error::RequestError};
use std::{
  collections::HashMap,
  fs::OpenOptions,
  io::{BufRead, BufReader},
  str::FromStr as _,
};

pub mod error;
pub mod http;
pub mod parsing;

#[cfg(test)]
mod test;

pub struct HttpRequest {
  pub url: String,
  pub method: String,
  pub headers: HashMap<String, String>,
  pub body: String,
  pub form: HashMap<String, MultipartRequestValue>,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum MultipartRequestValue {
  File(String),
  Text(String),
}

pub struct GraphQLRequest {
  pub url: String,
  pub query: String,
  pub headers: HashMap<String, String>,
}

#[derive(serde::Serialize)]
struct GraphQLQueryBody {
  pub query: String,
  pub variables: HashMap<String, String>,
}

impl TryFrom<GraphQLRequest> for reqwest::Request {
  type Error = crate::Error;

  fn try_from(value: GraphQLRequest) -> Result<Self, Self::Error> {
    let mut req = Request::new(
      Method::POST,
      Url::from_str(&value.url).map_err(|_| RequestError::ParseUrl)?,
    );
    let headers = req.headers_mut();

    for (name, value) in value.headers {
      headers.insert(
        HeaderName::from_str(&name).map_err(|_| RequestError::ParseHeader)?,
        HeaderValue::from_str(&value).map_err(|_| RequestError::ParseHeader)?,
      );
    }

    let body = req.body_mut();

    let query_body = GraphQLQueryBody {
      query: value.query,
      variables: Default::default(),
    };

    let body_query = serde_json::to_string(&query_body).unwrap();
    let parsed_body = Body::from(body_query);

    body.replace(parsed_body);
    Ok(req)
  }
}

impl TryFrom<HttpRequest> for reqwest::Request {
  type Error = crate::Error;

  fn try_from(value: HttpRequest) -> Result<Self, Self::Error> {
    let mut req = Request::new(
      Method::from_str(&value.method).map_err(RequestError::ParseMethod)?,
      Url::from_str(&value.url).map_err(|_| RequestError::ParseUrl)?,
    );
    let headers = req.headers_mut();

    for (name, value) in value.headers {
      headers.insert(
        HeaderName::from_str(&name).map_err(|_| RequestError::ParseHeader)?,
        HeaderValue::from_str(&value).map_err(|_| RequestError::ParseHeader)?,
      );
    }

    let body = req.body_mut();
    let parsed_body = Body::from(value.body);

    body.replace(parsed_body);
    Ok(req)
  }
}

impl HttpRequest {
  pub fn headers(&self) -> crate::Result<HeaderMap> {
    let headers = self
      .headers
      .iter()
      .map(|(key, value)| {
        Ok((
          HeaderName::from_str(key).map_err(|_| RequestError::ParseHeader)?,
          HeaderValue::from_str(value).map_err(|_| RequestError::ParseHeader)?,
        ))
      })
      .collect::<crate::Result<HeaderMap>>()?;

    Ok(headers)
  }
}

impl Lapse {
  pub fn get_raw_request_http(&self, name: &str) -> crate::Result<String> {
    let file_path = self.requests_path().join(name).with_extension("md");
    let f = OpenOptions::new()
      .read(true)
      .open(file_path)
      .map_err(RequestError::ReadRequestFile)?;

    let r = BufReader::new(f);

    let lines = r.lines();

    let http_lines = lines.take_while(|line| {
      let is_delimiter = line.as_ref().map(|inner| inner == "---").unwrap_or(true);

      !is_delimiter
    });

    let resolved_lines = http_lines
      .map(|line| {
        let resolved = line.map_err(|_| RequestError::ResolveHttpLine)?;

        Ok(resolved)
      })
      .collect::<crate::Result<Vec<_>>>()?;

    let http_content = resolved_lines.join("\n");

    Ok(http_content)
  }
}

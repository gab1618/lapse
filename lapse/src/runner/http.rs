use std::{
  collections::HashMap,
  time::{SystemTime, UNIX_EPOCH},
};

use reqwest::Client;

use crate::{
  env::hook::Event,
  request::{
    MultipartRequestValue,
    error::RequestError,
    parsing::{ParsedRequest, parse_request_http},
  },
  runner::{ExecutionResult, Runner},
};

impl Runner {
  pub async fn execute(&self, req: &str) -> crate::Result<ExecutionResult> {
    let start_time = SystemTime::now();
    let start_timestamp = start_time
      .duration_since(UNIX_EPOCH)
      .expect("Time should go forward")
      .as_nanos();

    if let Some(pre_request_hooks) = self.hooks.get(&Event::PreRequest) {
      for hook in pre_request_hooks {
        let loaded = self.runtime.load(hook);
        loaded.exec_async().await?;
      }
    }
    let resolved = self.eval(req)?;

    let client = Client::builder()
      .cookie_store(true)
      .build()
      .map_err(RequestError::CreateClient)?;

    let request = parse_request_http(&resolved, &self.default_scheme)?;

    let response = match request {
      ParsedRequest::Multipart(request) => {
        let mut form = reqwest::multipart::Form::new();

        let headers = request.headers()?;

        for (field, value) in request.body {
          match value {
            MultipartRequestValue::File(f) => {
              form = form
                .file(field, f)
                .await
                .map_err(|_| RequestError::AddFile)?;
            }
            MultipartRequestValue::Text(s) => {
              form = form.text(field, s);
            }
          }
        }

        client
          .post(request.url)
          .headers(headers)
          .multipart(form)
          .send()
          .await
          .map_err(RequestError::ExecuteRequest)?
      }
      ParsedRequest::Http(http_request) => {
        let parsed_request: reqwest::Request = http_request.try_into()?;

        client
          .execute(parsed_request)
          .await
          .map_err(RequestError::ExecuteRequest)?
      }
      ParsedRequest::GraphQL(graphql_request) => {
        let parsed_request: reqwest::Request = graphql_request.try_into()?;

        client
          .execute(parsed_request)
          .await
          .map_err(RequestError::ExecuteRequest)?
      }
    };

    let log_headers = response
      .headers()
      .iter()
      .map(|(name, value)| {
        let str_value = value
          .to_str()
          .map_err(RequestError::HeaderToStr)?
          .to_string();

        Ok((name.to_string(), str_value))
      })
      .collect::<crate::Result<HashMap<String, String>>>()?;

    let status_code = response.status().as_u16();
    let response_body = response
      .text()
      .await
      .map_err(RequestError::GetResponseBody)?;

    if let Some(post_request_hooks) = self.hooks.get(&Event::PostRequest) {
      for hook in post_request_hooks {
        let loaded = self.runtime.load(hook);
        loaded.exec_async().await?;
      }
    }

    let response = ExecutionResult {
      text: response_body,
      status: status_code,
      headers: log_headers,
      timestamp: start_timestamp,
      resolved_request: resolved,
      duration: start_time
        .elapsed()
        .expect("Time should go forward")
        .as_nanos(),
    };

    Ok(response)
  }
}

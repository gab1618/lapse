use std::collections::HashMap;

use reqwest::Client;

use crate::{
  eval::EvalCtx,
  log::ResponseLog,
  request::{
    error::RequestError,
    parsing::{MultipartRequestValue, ParsedRequest, parse_request_http},
  },
};

pub struct RequestRunner {
  ctx: EvalCtx,
}

impl RequestRunner {
  pub fn new(ctx: EvalCtx) -> Self {
    Self { ctx }
  }
  pub async fn execute(&self, req: &str) -> crate::Result<ResponseLog> {
    let resolved = self.ctx.eval(req)?;

    let client = Client::builder()
      .cookie_store(true)
      .build()
      .map_err(RequestError::CreateClient)?;

    let request = parse_request_http(&resolved)?;

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

    let log = ResponseLog {
      request: "request".to_string(),
      text: response_body,
      status: status_code,
      headers: log_headers,
    };

    Ok(log)
  }
}

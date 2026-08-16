use std::{
  collections::HashMap,
  fmt::Display,
  time::{SystemTime, UNIX_EPOCH},
};

use mlua::{IntoLua, Lua, Value};
use reqwest::Client;

use crate::{
  env::{EnvValue, hook::Event},
  lua::lexer::{DocumentLexer, DocumentToken},
  request::{
    error::RequestError,
    parsing::{MultipartRequestValue, ParsedRequest, parse_request_http},
  },
};

pub struct RequestRunner {
  runtime: Lua,
  hooks: HashMap<Event, Vec<String>>,
}

#[derive(Clone)]
pub struct RunnerResponse {
  pub text: String,
  pub status: u16,
  pub headers: HashMap<String, String>,
  pub timestamp: u128,
  pub duration: u128,
}

impl IntoLua for RunnerResponse {
  fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
    let table = lua.create_table()?;

    table.set("status", self.status)?;
    table.set("text", self.text)?;
    table.set("headers", self.headers)?;

    Ok(Value::Table(table))
  }
}

impl Display for RunnerResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{}", self.status)?;
    for (header, value) in &self.headers {
      writeln!(f, "{}: {}", header, value)?;
    }
    writeln!(f)?;
    writeln!(f, "{}", self.text)
  }
}

impl RequestRunner {
  pub fn new(runtime: Lua, hooks: HashMap<Event, Vec<String>>) -> Self {
    Self { runtime, hooks }
  }

  pub fn eval(&self, doc: &str) -> crate::Result<String> {
    let mut lexer = DocumentLexer::new(doc);
    let tokens = lexer.tokenize();
    let mut result = String::new();

    for token in tokens {
      match token {
        DocumentToken::String(inner) => {
          result.push_str(&inner);
        }
        DocumentToken::Expr(inner) => {
          let value: EnvValue = self.runtime.load(inner).eval()?;
          result.push_str(&value.to_string());
        }
      }
    }

    Ok(result)
  }
  pub async fn execute(&self, req: &str) -> crate::Result<RunnerResponse> {
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

    let response = RunnerResponse {
      text: response_body,
      status: status_code,
      headers: log_headers,
      timestamp: start_timestamp,
      duration: start_time
        .elapsed()
        .expect("Time should go forward")
        .as_nanos(),
    };

    Ok(response)
  }
}

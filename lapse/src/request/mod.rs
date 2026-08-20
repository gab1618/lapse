use crate::{Lapse, request::error::RequestError};
use std::{
  fs::OpenOptions,
  io::{BufRead, BufReader},
};

pub mod error;
pub mod http;
pub mod parsing;

#[cfg(test)]
mod test;

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

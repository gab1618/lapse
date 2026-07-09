use http::{Method, Request, Uri, Version};
use std::str::FromStr;

use crate::request::RequestFile;

impl RequestFile {
  // TODO: add proper error handling
  pub fn request(&self) -> Request<Vec<u8>> {
    let mut lines = self.http.lines();

    // --- 1. Parse the Request-Line ---
    let request_line = lines.next().ok_or("Empty request").unwrap();
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next().ok_or("Missing method").unwrap();
    let uri = request_parts.next().ok_or("Missing URI").unwrap();
    let version = request_parts.next().ok_or("Missing HTTP version").unwrap();

    let method = Method::from_str(method).unwrap();
    let uri = Uri::from_str(uri).unwrap();
    let version = match version {
      "HTTP/1.1" => Version::HTTP_11,
      "HTTP/1.0" => Version::HTTP_10,
      _ => panic!("Unsupported HTTP version"),
    };

    // --- 2. Parse Headers and Body ---
    let mut body_started = false;
    let mut headers = vec![];

    for line in lines.clone() {
      if !body_started && line.is_empty() {
        // An empty line marks the end of the headers and the start of the body.
        body_started = true;
        continue;
      }

      if !body_started {
        // Parse header line (e.g., "Header-Name: value")
        if let Some((name, value)) = line.split_once(':') {
          headers.push((name.trim(), value.trim()));
        }
      } else {
        // For this example, we collect the entire remaining string as the body.
        // A real implementation should handle the body as a stream.
        break;
      }
    }

    // Collect the remaining lines as the body (for simplicity)
    let body_string = if body_started {
      let body_lines = lines.collect::<Vec<&str>>();
      body_lines.join("\r\n")
    } else {
      String::new()
    };

    // --- 3. Build the hyper::Request ---
    let mut request_builder = Request::builder().method(method).uri(uri).version(version);

    for (name, value) in headers {
      request_builder = request_builder.header(name, value);
    }

    let request = request_builder.body(body_string.into_bytes()).unwrap();

    request
  }
}

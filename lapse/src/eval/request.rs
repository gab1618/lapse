use http::Request;

use crate::{Lapse, request::parsing::parse_request_http};

impl Lapse {
  pub fn resolve_request(&self, req: &str) -> crate::Result<Request<Vec<u8>>> {
    let ctx = self.get_eval_ctx()?;
    let resolved_tokens = ctx.eval(req)?;

    parse_request_http(resolved_tokens)
  }
}

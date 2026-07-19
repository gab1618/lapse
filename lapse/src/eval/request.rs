use http::Request;

use crate::{
  Lapse,
  parsing::RequestTokenizer,
  request::{RequestFile, http::parse_request_http},
};

impl Lapse {
  pub fn resolve_request(&self, req: &RequestFile) -> crate::Result<Request<Vec<u8>>> {
    let mut tokenizer = RequestTokenizer::new(&req.http);
    let tokens = tokenizer.tokenize();

    let ctx = self.get_eval_ctx()?;
    let resolved_tokens = ctx.eval(tokens)?;

    parse_request_http(resolved_tokens)
  }
}

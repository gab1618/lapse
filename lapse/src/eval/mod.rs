pub mod lexer;

#[cfg(test)]
mod test;

use crate::{
  Lapse,
  env::EnvValue,
  eval::lexer::{DocumentLexer, DocumentToken},
};
use mlua::Lua;

pub struct EvalCtx {
  runtime: Lua,
}

impl EvalCtx {
  pub fn new(runtime: Lua) -> Self {
    Self { runtime }
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
}

impl Lapse {
  pub fn get_eval_ctx(&self) -> crate::Result<EvalCtx> {
    let env = self
      .current_env()
      .map(|name| self.get_env(&name).unwrap_or_default())
      .unwrap_or_default();

    let secrets = self.load_secrets().unwrap_or_default();

    let runtime = Lua::new();

    runtime.globals().set("env", env.variables)?;
    runtime.globals().set("secret", secrets)?;

    Ok(EvalCtx::new(runtime))
  }
}

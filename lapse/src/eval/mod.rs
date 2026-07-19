pub mod request;

#[cfg(test)]
mod test;

use std::collections::HashMap;

use crate::{Lapse, env::EnvVariable, parsing::RequestToken};
use mlua::Lua;

pub struct EvalCtx {
  runtime: Lua,
}

impl EvalCtx {
  pub fn new(
    variables: HashMap<String, EnvVariable>,
    secrets: HashMap<String, EnvVariable>,
  ) -> crate::Result<Self> {
    let runtime = Lua::new();

    let env_table = runtime.create_table()?;

    for (key, value) in variables.into_iter() {
      env_table.set(key, value)?;
    }

    let secrets_table = runtime.create_table()?;

    for (key, value) in secrets.into_iter() {
      secrets_table.set(key, value)?;
    }

    runtime.globals().set("env", env_table)?;
    runtime.globals().set("secret", secrets_table)?;

    Ok(Self { runtime })
  }

  pub fn eval(&self, doc: Vec<RequestToken>) -> crate::Result<String> {
    let mut result = String::new();

    for token in doc {
      match token {
        RequestToken::String(inner) => {
          result.push_str(&inner);
        }
        RequestToken::Expr(inner) => {
          let value: EnvVariable = self.runtime.load(inner).eval()?;
          result.push_str(&value.to_string());
        }
      }
    }

    Ok(result)
  }
}

impl Lapse {
  pub fn get_eval_ctx(&self) -> crate::Result<EvalCtx> {
    let variables = self
      .current_env()
      .ok()
      .flatten()
      .map(|name| self.get_env(&name).unwrap_or_default())
      .unwrap_or_default();

    let secrets = self.load_secrets().unwrap_or_default();

    EvalCtx::new(variables, secrets)
  }
}


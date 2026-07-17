#[derive(Debug, thiserror::Error)]
pub enum EvalError {
  #[error("Could not evaluate script: {0}")]
  EvaluateScript(#[source] mlua::Error),
}

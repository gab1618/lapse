use crate::{
  Lapse,
  log::ResponseLog,
  runner::{RequestRunner, Response},
};

impl Lapse {
  fn save_runner_log(&self, log: Response, name: String) -> crate::Result<ResponseLog> {
    let log = ResponseLog {
      request: name,
      text: log.text,
      status: log.status,
      headers: log.headers,
      duration: log.duration,
      timestamp: log.timestamp,
    };

    self.save_log(&log)?;

    Ok(log)
  }
  pub async fn request(&self, name: &str) -> crate::Result<ResponseLog> {
    let runtime = self.get_runtime()?;
    let hooks = self.get_hooks_scripts()?;

    let req = self.get_raw_request_http(name)?;
    let env = self.get_env(&self.current_env()).unwrap_or_default();
    let runner = RequestRunner::new(runtime, hooks, env.config.default_scheme.to_string());

    let response = runner.execute(&req).await?;

    let log = self.save_runner_log(response.clone(), name.to_string())?;

    Ok(log)
  }
}

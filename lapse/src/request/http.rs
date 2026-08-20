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
    let req = self.get_raw_request_http(name)?;
    let runner = RequestRunner::from_space(self)?;

    let response = runner.execute(&req).await?;

    let log = self.save_runner_log(response.clone(), name.to_string())?;

    Ok(log)
  }
}

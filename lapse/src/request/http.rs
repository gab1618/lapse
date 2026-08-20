use crate::{
  Lapse,
  log::ResponseLog,
  runner::{Response, Runner},
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
    let runner = Runner::from_space(self)?;

    let response = runner.execute(&req).await?;

    let log = self.save_runner_log(response, name.to_string())?;

    Ok(log)
  }
}

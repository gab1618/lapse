use std::{fmt::Display, time::Duration};

use chrono::{DateTime, Local};
use lapse::log::ResponseLog;

use colored::{Color, Colorize as _};

pub struct DetailedLogEntry(pub ResponseLog);
pub struct InlineLogEntry(pub ResponseLog);

fn status_color(status: u16) -> colored::Color {
  match status {
    200 | 201 | 204 => Color::Green,
    500 | 503 | 400 | 404 | 403 | 401 => Color::Red,
    _ => Color::Blue,
  }
}
fn method_color(method: &str) -> colored::Color {
  match method {
    "GET" => Color::Green,
    "DELETE" => Color::Red,
    "POST" => Color::Blue,
    "PATCH" => Color::Yellow,
    "PUT" => Color::Yellow,
    _ => Color::White,
  }
}

impl Display for InlineLogEntry {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut request_lines = self.0.result.resolved_request.lines();
    let request_head = request_lines.next().unwrap_or_default();
    if let Some((method, url)) = request_head.split_once(' ') {
      write!(f, "{} {}", method.color(method_color(method)), url)?;
    }

    Ok(())
  }
}

impl Display for DetailedLogEntry {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{} {}", "Request".black(), self.0.request.green())?;

    let curr_time = Local::now();
    let curr_timezone = curr_time.timezone();

    let dt =
      DateTime::from_timestamp_nanos(self.0.result.timestamp as i64).with_timezone(&curr_timezone);
    let formated_date = dt.format("%d-%m-%Y %H:%M:%S").to_string();
    writeln!(f, "{} {}", "Date:".black(), formated_date.cyan())?;

    let base_duration = Duration::from_nanos_u128(self.0.result.duration);
    let duration_milis = base_duration.as_millis();
    writeln!(
      f,
      "{} {}{}",
      "Duration:".black(),
      duration_milis.to_string().purple(),
      "ms".purple()
    )?;
    writeln!(
      f,
      "{} {}\n",
      "Status:".black(),
      self
        .0
        .result
        .status
        .to_string()
        .color(status_color(self.0.result.status))
    )?;

    writeln!(f, "{}\n", "Headers".red())?;

    for (key, val) in &self.0.result.headers {
      writeln!(f, "{}: {}", key.bright_black(), val.yellow())?;
    }

    writeln!(f)?;
    writeln!(f, "{}", self.0.result.text)?;

    Ok(())
  }
}

use std::{
  fmt::{Display, Write as _},
  io,
  sync::{Arc, Mutex, MutexGuard},
  time::Duration,
};

use chrono::{DateTime, Local};
use colored::{Color, Colorize as _};

use is_terminal::IsTerminal;
use lapse::log::{ResponseLog, iter::ResponseLogsIter};

use crate::command::{log::error::LogError, open_lapse};

use minus::{Pager, hooks::Hook};

pub mod error;

pub struct FormatedLogEntry(pub ResponseLog);

impl FormatedLogEntry {
  pub fn new(src: ResponseLog) -> Self {
    Self(src)
  }
}

fn status_color(status: u16) -> colored::Color {
  match status {
    200 | 201 | 204 => Color::Green,
    500 | 503 | 400 | 404 | 403 | 401 => Color::Red,
    _ => Color::Blue,
  }
}

impl Display for FormatedLogEntry {
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

fn append_to_log_until_fills(
  thread_pager: &mut Pager,
  mut missing_lines: usize,
  entries: &mut MutexGuard<'_, ResponseLogsIter>,
) -> crate::Result<()> {
  while missing_lines > 0 {
    if let Some(new_log) = entries.next() {
      let formated = FormatedLogEntry::new(new_log);
      let content = format!("{formated}");
      let lines_written = content.lines().count() + 1;
      missing_lines -= lines_written;
      writeln!(thread_pager, "{}", content).map_err(LogError::SendToPager)?;
    } else {
      break;
    }
  }

  Ok(())
}

pub fn log() -> crate::Result<()> {
  let lapse = Arc::new(open_lapse()?);

  if io::stdout().is_terminal() {
    let entries_iter = lapse.logs_iter().into_parsed();

    let shared_entries_iter = Arc::new(Mutex::new(entries_iter));
    let pager = Pager::new();
    pager
      .set_run_no_overflow(true)
      .map_err(LogError::SetupPagerConfig)?;

    let mut thread_pager = pager.clone();
    let thread_entries_iter = shared_entries_iter.clone();

    pager
      .add_hook(
        Hook::EofReached,
        0,
        Box::new(move |state| {
          let mut entries = thread_entries_iter.lock().expect("Poisoned mutex");
          let missing_lines = state.rows - state.screen.line_count().min(state.rows);
          append_to_log_until_fills(&mut thread_pager, missing_lines, &mut entries)
            .expect("Error fetching log content");
        }),
      )
      .map_err(LogError::SetupPagerConfig)?;

    let mut thread_pager = pager.clone();
    let thread_entries_iter = shared_entries_iter.clone();

    pager
      .add_hook(
        Hook::PrePagerStart,
        1,
        Box::new(move |state| {
          let mut entries = thread_entries_iter.lock().expect("Poisoned mutex");
          let missing_lines = state.rows - state.screen.line_count().min(state.rows);
          append_to_log_until_fills(&mut thread_pager, missing_lines, &mut entries)
            .expect("Error fetching log content");
        }),
      )
      .map_err(LogError::SetupPagerConfig)?;

    minus::page_all(pager).map_err(LogError::SetupPagerConfig)?;
  } else {
    let entries_iter = lapse.logs_iter().into_parsed();
    let pretty_entries = entries_iter.map(FormatedLogEntry);
    for entry in pretty_entries {
      println!("{entry}");
    }
  }

  Ok(())
}

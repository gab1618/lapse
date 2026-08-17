use std::{
  fmt::{Display, Write as _},
  io,
  sync::{Arc, Mutex, MutexGuard},
  time::Duration,
};

use chrono::DateTime;
use colored::Colorize as _;

use is_terminal::IsTerminal;
use lapse::{
  log::{ResponseLog, ResponseLogsIter},
  tree::{FlatTreeConfig, resource::Resource},
};

use crate::{
  command::{log::error::LogError, open_lapse},
  select::select_tree_entry,
};

use minus::{Pager, hooks::Hook};

pub mod error;

pub struct FormatedLogEntry(ResponseLog);

impl FormatedLogEntry {
  pub fn new(src: ResponseLog) -> Self {
    Self(src)
  }
}

impl Display for FormatedLogEntry {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{} {}", "Request".black(), self.0.request.green())?;

    let dt = DateTime::from_timestamp_nanos(self.0.timestamp as i64);
    let formated_date = dt.format("%d-%m-%Y %H:%M:%S").to_string();
    writeln!(f, "{} {}", "Date:".black(), formated_date.cyan())?;

    let base_duration = Duration::from_nanos_u128(self.0.duration);
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
      self.0.status.to_string().blue()
    )?; // TODO: add color based on actual status

    writeln!(f, "{}\n", "Headers".red())?;

    for (key, val) in &self.0.headers {
      writeln!(f, "{}: {}", key.bright_black(), val.yellow())?;
    }

    writeln!(f)?;
    writeln!(f, "{}", self.0.text)?;

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

pub fn log(request: Option<String>) -> crate::Result<()> {
  let lapse = Arc::new(open_lapse()?);
  let tree = lapse.get_resource_tree(Resource::Requests, None)?;

  let flatlist_config = FlatTreeConfig::default();
  let selected_request = select_tree_entry(&tree, request, flatlist_config)?;

  if io::stdout().is_terminal() {
    let entries_iter = lapse.logs_iter(&selected_request).into_parsed();

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
    let entries_iter = lapse.logs_iter(&selected_request).into_parsed();
    let pretty_entries = entries_iter.map(FormatedLogEntry);
    for entry in pretty_entries {
      println!("{entry}");
    }
  }

  Ok(())
}

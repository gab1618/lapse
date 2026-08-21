use std::{
  fmt::Write as _,
  io,
  sync::{Arc, Mutex, MutexGuard},
};

use is_terminal::IsTerminal;
use lapse::log::iter::ResponseLogsIter;

use crate::command::{
  log::{display::InlineLogEntry, error::LogError},
  open_lapse,
};

use minus::{Pager, hooks::Hook};

pub mod display;
pub mod error;

fn append_to_log_until_fills(
  thread_pager: &mut Pager,
  mut missing_lines: usize,
  entries: &mut MutexGuard<'_, ResponseLogsIter>,
) -> crate::Result<()> {
  while missing_lines > 0 {
    if let Some(new_log) = entries.next() {
      let formated = InlineLogEntry(new_log);
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
    let pretty_entries = entries_iter.map(InlineLogEntry);
    for entry in pretty_entries {
      println!("{entry}");
    }
  }

  Ok(())
}

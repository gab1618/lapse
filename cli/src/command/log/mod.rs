use std::{
  fmt::Write as _,
  io,
  sync::{Arc, Mutex, MutexGuard},
};

use is_terminal::IsTerminal;
use lapse::{log::ResponseLogsIter, tree::resource::Resource};

use crate::{
  collection::FlatlistReadConfig,
  command::{log::error::LogError, open_lapse},
  select::select_tree_entry,
};

use minus::{Pager, hooks::Hook};

pub mod error;

fn append_to_log_until_fills(
  thread_pager: &mut Pager,
  mut missing_lines: usize,
  entries: &mut MutexGuard<'_, ResponseLogsIter>,
) -> crate::Result<()> {
  while missing_lines > 0 {
    if let Some((entry_name, new_log)) = entries.next() {
      let lines_written = new_log.lines().count() + 1;
      missing_lines -= lines_written;
      writeln!(thread_pager, "{}", entry_name).map_err(LogError::SendToPager)?;
      writeln!(thread_pager, "{}", new_log).map_err(LogError::SendToPager)?;
    } else {
      break;
    }
  }

  Ok(())
}

pub fn log(request: Option<String>) -> crate::Result<()> {
  let lapse = Arc::new(open_lapse()?);
  let tree = lapse.get_resource_tree(Resource::Requests, None)?;

  let flatlist_config = FlatlistReadConfig::default().files(true);
  let selected_request = select_tree_entry(&tree, request, flatlist_config)?;

  if io::stdout().is_terminal() {
    let entries_iter = lapse.logs_iter(&selected_request)?;

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
    let entries_iter = lapse.logs_iter(&selected_request)?;
    for (name, entry) in entries_iter {
      println!("{name}");
      println!("{entry}");
    }
  }

  Ok(())
}

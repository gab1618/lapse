use std::{
  fmt::Write as _,
  sync::{Arc, Mutex},
};

use lapse::tree::resource::Resource;

use crate::{command::open_lapse, select::select_tree_entry};

use minus::{Pager, hooks::Hook};

pub fn log(request: Option<String>) -> crate::Result<()> {
  let lapse = Arc::new(open_lapse()?);
  let tree = lapse.get_resource_tree(Resource::Requests, None)?;

  let selected_request = select_tree_entry(&tree, request)?;

  let entries_iter = lapse.logs_iter(&selected_request)?;

  let shared_entries_iter = Arc::new(Mutex::new(entries_iter));

  let pager = Pager::new();
  pager.set_run_no_overflow(true).unwrap();

  let mut thread_pager = pager.clone();
  let thread_entries_iter = shared_entries_iter.clone();

  pager
    .add_hook(
      Hook::EofReached,
      0,
      Box::new(move |_state| {
        let mut entries = thread_entries_iter.lock().unwrap();
        if let Some((entry_name, new_log)) = entries.next() {
          writeln!(thread_pager, "{}", entry_name).unwrap();
          writeln!(thread_pager, "{}", new_log).unwrap();
        }
      }),
    )
    .unwrap();

  let mut thread_pager = pager.clone();
  let thread_entries_iter = shared_entries_iter.clone();

  pager
    .add_hook(
      Hook::PrePagerStart,
      1,
      Box::new(move |state| {
        let mut entries = thread_entries_iter.lock().unwrap();
        let mut missing_lines = state.rows - state.screen.line_count().min(state.rows);

        while missing_lines > 0 {
          if let Some((entry_name, new_log)) = entries.next() {
            let lines_written = new_log.lines().count() + 1;
            missing_lines -= lines_written;
            writeln!(thread_pager, "{}", entry_name).unwrap();
            writeln!(thread_pager, "{}", new_log).unwrap();
          } else {
            break;
          }
        }
      }),
    )
    .unwrap();

  minus::page_all(pager).unwrap();

  Ok(())
}

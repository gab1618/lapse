use std::{
  collections::BTreeSet,
  fs::{self, DirEntry},
  str::FromStr as _,
};

use crate::{Lapse, log::ResponseLog};

#[derive(Clone)]
pub struct RawResponseLogsIter {
  lapse: Lapse,
  src: Vec<String>,
}
impl RawResponseLogsIter {
  pub fn into_parsed(self) -> ResponseLogsIter {
    ResponseLogsIter::new(self)
  }
}

pub struct ResponseLogsIter {
  src: RawResponseLogsIter,
}
impl ResponseLogsIter {
  pub fn new(src: RawResponseLogsIter) -> Self {
    Self { src }
  }
}

impl Iterator for ResponseLogsIter {
  type Item = ResponseLog;

  fn next(&mut self) -> Option<Self::Item> {
    let raw = self.src.next()?;
    let parsed = ResponseLog::from_str(&raw).ok()?;

    Some(parsed)
  }
}

impl Iterator for RawResponseLogsIter {
  type Item = String;

  fn next(&mut self) -> Option<Self::Item> {
    let name = self.src.pop()?;
    let full_entry_path = self.lapse.logs_path().join(&name);

    let contents = fs::read_to_string(full_entry_path).ok()?;

    Some(contents)
  }
}

impl Lapse {
  pub fn logs_iter(&self) -> RawResponseLogsIter {
    let request_logs_path = self.logs_path();

    let entries: Vec<std::io::Result<DirEntry>> = fs::read_dir(request_logs_path)
      .map(|inner| inner.collect())
      .unwrap_or_default();

    let entries_paths = entries
      .into_iter()
      .filter_map(|entry| entry.ok().map(|sub| sub.path()));

    let valid_entries = entries_paths.filter(|path| !path.is_dir());

    let ordered_entries_names = valid_entries
      .filter_map(|entry| {
        let file_name = entry.file_name()?;
        let str_name = file_name.to_str()?;
        Some(str_name.to_string())
      })
      .collect::<BTreeSet<_>>();

    let entries_names: Vec<String> = ordered_entries_names.into_iter().collect();

    RawResponseLogsIter {
      lapse: self.clone(),
      src: entries_names,
    }
  }
}

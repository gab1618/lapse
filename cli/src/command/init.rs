use std::{fs, path::PathBuf};

use lapse::Lapse;
use lapse_template::templates::{LapsePreset, openapi::OpenApi};

use crate::{Error, cli::AvailablePreset};

pub fn init(preset: Option<AvailablePreset>, schema: Option<PathBuf>) -> crate::Result<()> {
  let curr_dir = std::env::current_dir().map_err(Error::GetCurrentDir)?;
  Lapse::init(&curr_dir)?;

  let selected_preset = schema
    .and_then(|schema_path| {
      let raw_schema = fs::read_to_string(schema_path).ok()?;

      let schema = OpenApi::from_str_schema(&raw_schema).ok()?;
      let as_preset: LapsePreset = schema.try_into().ok()?;

      Some(as_preset)
    })
    .unwrap_or_else(|| {
      preset
        .map(|entry| match entry {
          AvailablePreset::Httpbin => LapsePreset::httpbin(),
        })
        .unwrap_or_default()
    });

  selected_preset.load(curr_dir)?;

  println!("Initialized Lapse space");

  Ok(())
}

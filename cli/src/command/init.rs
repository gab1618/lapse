use std::fs;

use lapse::Lapse;
use lapse_template::templates::{LapsePreset, openapi::OpenApi};

use crate::{Error, cli::AvailablePreset};

pub fn init(preset: Option<AvailablePreset>, schema: Option<String>) -> crate::Result<()> {
  let curr_dir = std::env::current_dir().map_err(Error::GetCurrentDir)?;
  Lapse::init(&curr_dir)?;

  let selected_preset = schema
    .map(|subpath| {
      let schema_path = curr_dir.join(subpath);
      let raw_schema = fs::read_to_string(schema_path).ok()?;

      let schema = OpenApi::from_str_schema(&raw_schema).unwrap();
      let as_preset: LapsePreset = schema.into();

      Some(as_preset)
    })
    .flatten()
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

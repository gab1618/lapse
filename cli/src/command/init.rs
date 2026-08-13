use lapse::Lapse;
use lapse_template::templates::LapsePreset;

use crate::{Error, cli::AvailablePreset};

pub fn init(preset: Option<AvailablePreset>) -> crate::Result<()> {
  let curr_dir = std::env::current_dir().map_err(Error::GetCurrentDir)?;
  Lapse::init(&curr_dir)?;

  let selected_preset = preset
    .map(|entry| match entry {
      AvailablePreset::Httpbin => LapsePreset::httpbin(),
    })
    .unwrap_or_default();

  selected_preset.load(curr_dir)?;

  println!("Initialized Lapse space");

  Ok(())
}

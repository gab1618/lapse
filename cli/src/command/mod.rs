use lapse::{Lapse, tree::resource::Resource};
use lapse_template::templates::LapsePreset;

use crate::{Error, cli::AvailablePreset, collection::output_tree};

pub mod env;
pub mod log;
pub mod script;
pub mod send;

pub fn open_lapse() -> crate::Result<Lapse> {
  let curr_dir = std::env::current_dir().map_err(Error::GetCurrentDir)?;
  Ok(Lapse::open(curr_dir)?)
}

pub fn init(preset: Option<AvailablePreset>) -> crate::Result<()> {
  let curr_dir = std::env::current_dir().map_err(Error::GetCurrentDir)?;
  Lapse::init(&curr_dir)?;

  let selected_preset = preset
    .map(|entry| match entry {
      AvailablePreset::Httpbin => LapsePreset::httpbin(),
    })
    .unwrap_or_default();

  selected_preset.load(curr_dir).unwrap();

  println!("Initialized Lapse space");

  Ok(())
}

pub fn ls(path: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let collection = lapse.get_resource_tree(Resource::Requests, path)?;
  output_tree(0, &collection);

  Ok(())
}

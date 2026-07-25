use lapse::{Lapse, tree::resource::Resource};

use crate::{Error, collection::output_tree};

pub mod env;
pub mod script;
pub mod send;

pub fn open_lapse() -> crate::Result<Lapse> {
  let curr_dir = std::env::current_dir().map_err(Error::GetCurrentDir)?;
  Ok(Lapse::open(curr_dir)?)
}

pub fn init() -> crate::Result<()> {
  let curr_dir = std::env::current_dir().map_err(Error::GetCurrentDir)?;
  Lapse::init(curr_dir)?;
  println!("Initialized Lapse space");

  Ok(())
}

pub fn ls(path: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let collection = lapse.get_resource_tree(Resource::Requests, path)?;
  output_tree(0, &collection);

  Ok(())
}

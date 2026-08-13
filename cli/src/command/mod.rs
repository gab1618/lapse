use lapse::{Lapse, tree::resource::Resource};

use crate::{
  Error,
  collection::{FlatlistReadConfig, output_tree},
};

pub mod env;
pub mod init;
pub mod log;
pub mod script;
pub mod send;

pub fn open_lapse() -> crate::Result<Lapse> {
  let curr_dir = std::env::current_dir().map_err(Error::GetCurrentDir)?;
  Ok(Lapse::open(curr_dir)?)
}

pub fn ls(path: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let collection = lapse.get_resource_tree(Resource::Requests, path)?;

  let flatlist_config = FlatlistReadConfig::default().files(true).dirs(true);
  output_tree(0, &collection, flatlist_config);

  Ok(())
}

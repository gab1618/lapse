use lapse::{
  Lapse,
  tree::{FlatTreeConfig, resource::Resource},
};

use crate::{Error, collection::output_tree};

pub mod config;
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

  let flatlist_config = FlatTreeConfig::default().include_dirs(true);
  output_tree(&collection, flatlist_config);

  Ok(())
}

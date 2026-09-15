use lapse::Lapse;

use crate::Error;

pub mod env;
pub mod init;
pub mod log;
pub mod ls;
pub mod script;
pub mod send;

pub fn open_lapse() -> crate::Result<Lapse> {
  let curr_dir = std::env::current_dir().map_err(Error::GetCurrentDir)?;
  Ok(Lapse::open(curr_dir)?)
}

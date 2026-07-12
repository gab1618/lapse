use std::{
  fs::{self, OpenOptions},
  io::{Read, Write},
};

use crate::{Lapse, state::error::StateError};

pub mod error;

impl Lapse {
  fn ensure_state_dir(&self) -> crate::Result<()> {
    let base_state_path = self.state_path();
    fs::create_dir_all(&base_state_path).map_err(StateError::EnsureStateDir)?;

    Ok(())
  }
  pub fn set_state(&self, key: &str, value: &str) -> crate::Result<()> {
    let base_state_path = self.state_path();

    self.ensure_state_dir()?;

    let mut f = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(base_state_path.join(key))
      .map_err(StateError::OpenStateFile)?;

    f.write_all(value.as_bytes())
      .map_err(StateError::SaveState)?;
    Ok(())
  }

  pub fn get_state(&self, key: &str) -> crate::Result<Option<String>> {
    let base_state_path = self.state_path();
    self.ensure_state_dir()?;

    let f = OpenOptions::new()
      .read(true)
      .open(base_state_path.join(key));

    match f {
      Ok(mut f) => {
        let mut buf = String::new();
        f.read_to_string(&mut buf).map_err(StateError::ReadState)?;

        Ok(Some(buf))
      }
      Err(_) => Ok(None),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::test::TempLapse;

  #[test]
  fn test_set_and_get_state() {
    let lapse = TempLapse::new();

    lapse.set_state("env", "default").unwrap();
    lapse.set_state("name", "John").unwrap();

    assert_eq!(lapse.get_state("env").unwrap(), Some("default".to_string()));
    assert_eq!(lapse.get_state("name").unwrap(), Some("John".to_string()));
    assert_eq!(lapse.get_state("other").unwrap(), None);
  }
}

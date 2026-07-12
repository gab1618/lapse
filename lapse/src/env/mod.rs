use std::{
  fs::{self, OpenOptions},
  io::{BufReader, Read, Write},
};

use crate::Lapse;

#[cfg(test)]
mod test;

impl Lapse {
  // TODO: later, check the existence of the env
  pub fn switch_env(&self, name: &str) {
    let state_path = self.state_path();
    fs::create_dir_all(&state_path).unwrap();
    let env_state_file = state_path.join("env");

    let mut f = OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open(env_state_file)
      .unwrap();

    f.write_all(name.as_bytes()).unwrap();
  }

  // TODO: not being in a env is a thing
  pub fn current_env(&self) -> String {
    let state_path = self.state_path();
    fs::create_dir_all(&state_path).unwrap();
    let env_state_file = state_path.join("env");

    let f = OpenOptions::new().read(true).open(env_state_file).unwrap();
    let mut r = BufReader::new(f);

    let mut curr_env = String::new();
    r.read_to_string(&mut curr_env).unwrap();

    curr_env
  }
}

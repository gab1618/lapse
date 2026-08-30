use lapse_config::Config;

pub fn set(name: &str, value: &str) -> crate::Result<()> {
  let mut config = Config::read();
  config.insert(name.into(), value.into());

  config.save();
  Ok(())
}

pub fn get(name: &str) -> crate::Result<()> {
  let config = Config::read();

  if let Some(value) = config.get(name) {
    println!("{value}");
  }
  Ok(())
}

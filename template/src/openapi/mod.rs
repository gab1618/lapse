use crate::Generator;

pub struct OpenApiSpec {}

impl Generator for OpenApiSpec {
  fn load<P: AsRef<std::path::Path>>(&self, path: P) -> crate::Result<()> {
    let base_path = path.as_ref();

    println!("{}", base_path.display());

    Ok(())
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn test_parse_example() {}
}

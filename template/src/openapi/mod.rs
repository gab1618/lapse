use openapi::Spec;

use crate::Generator;

impl Generator for Spec {
  fn load<P: AsRef<std::path::Path>>(&self, path: P) -> crate::Result<()> {
    Ok(())
  }
}

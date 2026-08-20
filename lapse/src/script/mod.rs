pub mod error;

use std::fs;

use crate::{Lapse, runner::Runner, script::error::ScriptError};

impl Lapse {
  pub async fn run_script(&self, name: &str) -> crate::Result<()> {
    let script_path = self.scripts_path().join(name).with_extension("lua");

    let script_content = fs::read_to_string(script_path).map_err(ScriptError::ReadScriptFile)?;
    let runner = Runner::from_space(self)?;

    runner.run(&script_content).await?;

    Ok(())
  }
}

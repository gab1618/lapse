use std::io::Write;

use clap::CommandFactory;
use clap_complete::Shell;

use crate::cli::Cli;

#[derive(Clone, clap::ValueEnum)]
pub enum CliCompletionShell {
  Bash,
  Elvish,
  Zsh,
  Fish,
  Powershell,
}

impl Into<Shell> for CliCompletionShell {
  fn into(self) -> Shell {
    match self {
      CliCompletionShell::Bash => Shell::Bash,
      CliCompletionShell::Elvish => Shell::Elvish,
      CliCompletionShell::Zsh => Shell::Zsh,
      CliCompletionShell::Fish => Shell::Fish,
      CliCompletionShell::Powershell => Shell::PowerShell,
    }
  }
}

pub fn generate_completion<W: Write>(s: CliCompletionShell, w: &mut W) {
  let shell: Shell = s.into();
  clap_complete::generate(shell, &mut Cli::command(), env!("CARGO_BIN_NAME"), w);
}

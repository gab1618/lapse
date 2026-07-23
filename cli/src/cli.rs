use clap::{Parser, Subcommand};

use crate::completion::CliCompletionShell;

#[derive(Subcommand)]
pub enum Command {
  Init,
  Ls,
  Send {
    request: Option<String>,
  },
  Run {
    script: Option<String>,
  },
  Completion {
    shell: CliCompletionShell,
  },
  #[command(subcommand)]
  Env(EnvCommand),
}

#[derive(Parser)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

#[derive(Subcommand)]
pub enum EnvCommand {
  Switch { name: Option<String> },
  Ls,
}

use clap::{Parser, Subcommand};

use crate::completion::CliCompletionShell;

#[derive(Subcommand)]
pub enum Command {
  Init,
  Ls {
    path: Option<String>,
  },
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
  #[command(subcommand)]
  Script(ScriptCommand),
}

#[derive(Parser)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

#[derive(Subcommand)]
pub enum EnvCommand {
  Switch { name: Option<String> },
  Ls { path: Option<String> },
}

#[derive(Subcommand)]
pub enum ScriptCommand {
  Run { script: Option<String> },
  Ls { path: Option<String> },
}

use clap::{Parser, Subcommand};

use crate::completion::CliCompletionShell;

#[derive(Subcommand)]
pub enum Command {
  Init,
  Ls,
  Send { request: Option<String> },
  Completion { shell: CliCompletionShell },
}

#[derive(Parser)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

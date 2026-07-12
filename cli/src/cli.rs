use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Command {
  Init,
  Ls,
  Send { request: Option<String> },
}

#[derive(Parser)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

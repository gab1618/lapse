use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Command {
  Init,
}

#[derive(Parser)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

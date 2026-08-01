use clap::{Parser, Subcommand};

use crate::completion::CliCompletionShell;

/// Feature-rich Http CLI focused on UX
#[derive(Parser)]
#[command(version, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
  /// Initializes a Lapse space at current dir
  Init,
  /// Lists all requests
  #[command(about)]
  Ls {
    /// Base path to list from
    path: Option<String>,
  },
  /// Sends a request
  #[command(about)]
  Send { request: Option<String> },
  /// Runs a script
  #[command(about)]
  Run { script: Option<String> },
  /// Outputs a completion script
  #[command(about)]
  Completion { shell: CliCompletionShell },
  /// Environment commands
  #[command(subcommand, about)]
  Env(EnvCommand),
  /// Script commands
  #[command(subcommand, about)]
  Script(ScriptCommand),
}

#[derive(Subcommand)]
pub enum EnvCommand {
  #[command(about)]
  /// Switches to another env
  Switch {
    /// Name of the environment
    name: Option<String>,
  },
  /// Lists all environments
  Ls {
    /// Base path to list environments
    path: Option<String>,
  },
}

#[derive(Subcommand)]
pub enum ScriptCommand {
  /// Runs a script
  Run {
    /// The script name
    script: Option<String>,
  },
  /// Lists all scripts
  Ls {
    /// The base path to list from
    path: Option<String>,
  },
}

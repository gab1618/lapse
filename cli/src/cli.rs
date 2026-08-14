use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::completion::CliCompletionShell;

/// Feature-rich Http CLI focused on UX
#[derive(Parser)]
#[command(version, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

#[derive(Clone, clap::ValueEnum)]
pub enum AvailablePreset {
  Httpbin,
}

#[derive(Subcommand)]
pub enum Command {
  /// Initializes a Lapse space at current dir
  Init {
    #[arg(short, long)]
    preset: Option<AvailablePreset>,
    #[arg(short, long)]
    schema: Option<PathBuf>,
  },
  /// Lists all requests
  Ls {
    /// Base path to list from
    path: Option<String>,
  },
  /// Sends a request
  Send { request: Option<String> },
  /// Runs a script
  Run { script: Option<String> },
  /// Logs a request's response logs history
  Log {
    /// Request name
    request: Option<String>,
  },
  /// Outputs a completion script
  Completion { shell: CliCompletionShell },
  /// Environment commands
  #[command(subcommand)]
  Env(EnvCommand),
  /// Script commands
  #[command(subcommand)]
  Script(ScriptCommand),
}

#[derive(Subcommand)]
pub enum EnvCommand {
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

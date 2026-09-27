mod cli;
mod collection;
mod command;
mod completion;
mod error;
mod select;

pub use error::{Error, Result};

use std::io::stdout;

use clap::Parser;
use cli::Cli;

use crate::{
  cli::{Command, ScriptCommand},
  completion::generate_completion,
};

#[tokio::main]
async fn main() {
  if let Err(err) = entrypoint().await {
    eprintln!("{}", err);
  }
}

pub async fn execute_cli(args: Cli) -> error::Result<()> {
  match args.command {
    Command::Init { preset, schema } => {
      command::init::init(preset, schema)?;
    }
    Command::Ls { path } => {
      command::ls(path)?;
    }
    Command::Send {
      request,
      body,
      dry_run,
    } => {
      command::send::send(request, body, dry_run).await?;
    }
    Command::Completion { shell } => {
      let mut out = stdout();
      generate_completion(shell, &mut out);
    }
    Command::Env(env_command) => match env_command {
      cli::EnvCommand::Switch { name } => {
        command::env::switch(name)?;
      }
      cli::EnvCommand::Ls { path } => {
        command::env::ls(path)?;
      }
      cli::EnvCommand::Unset => {
        command::env::unset()?;
      }
    },
    Command::Run { script } => {
      command::script::run(script).await?;
    }
    Command::Script(script_command) => match script_command {
      ScriptCommand::Run { script } => {
        command::script::run(script).await?;
      }
      ScriptCommand::Ls { path } => {
        command::script::ls(path)?;
      }
    },
    Command::Log { entry } => command::log::log(entry)?,
  }
  Ok(())
}

async fn entrypoint() -> error::Result<()> {
  let args = Cli::parse();

  execute_cli(args).await?;

  Ok(())
}

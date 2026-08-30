mod cli;
mod collection;
mod command;
mod completion;
mod error;
mod select;

pub use error::{Error, Result};
use lapse::{log::ResponseLog, runner::Runner};

use std::io::stdout;

use clap::Parser;
use cli::Cli;

use crate::{
  cli::{Command, ScriptCommand},
  command::{log::display::DetailedLogEntry, open_lapse},
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
    Command::Config(config_command) => match config_command {
      cli::ConfigCommand::Get { key } => command::config::get(&key)?,
      cli::ConfigCommand::Set { key, value } => command::config::set(&key, &value)?,
    },
  }
  Ok(())
}

async fn entrypoint() -> error::Result<()> {
  use clap::error::ErrorKind;

  match Cli::try_parse() {
    Ok(args) => execute_cli(args).await?,
    Err(err) => {
      if err.kind() != ErrorKind::InvalidSubcommand {
        err.print().unwrap();
        return Ok(());
      }

      let args_iter = std::env::args().skip(1);
      let args: Vec<_> = args_iter.collect();
      let request = args.join(" ");

      let runner = Runner::standalone();
      let result = runner.execute(&request).await?;

      let log = ResponseLog {
        request: None,
        result,
      };

      if let Ok(lapse) = open_lapse() {
        lapse.save_log(&log)?;
      }
      let detailed_log = DetailedLogEntry(log);
      print!("{detailed_log}");
    }
  }

  Ok(())
}

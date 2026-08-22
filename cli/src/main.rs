mod cli;
mod collection;
mod command;
mod completion;
mod error;
mod select;

pub use error::{Error, Result};
use lapse::runner::Runner;

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

async fn entrypoint() -> error::Result<()> {
  if let Ok(args) = Cli::try_parse() {
    match args.command {
      Command::Init { preset, schema } => {
        command::init::init(preset, schema)?;
      }
      Command::Ls { path } => {
        command::ls(path)?;
      }
      Command::Send { request } => {
        command::send::send(request).await?;
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
  } else {
    let args_iter = std::env::args().skip(1);
    let args: Vec<_> = args_iter.collect();
    let request = args.join(" ");

    let runner = Runner::standalone();
    let result = runner.execute(&request).await?;

    println!("{result}");
  }

  Ok(())
}

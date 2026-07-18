mod cli;
mod collection;
mod completion;
mod error;

use std::io::stdout;

use clap::Parser;
use cli::Cli;

use lapse::Lapse;

use crate::{
  cli::Command,
  collection::{get_requests_flatlist, output_requests_collection},
  completion::generate_completion,
};

use inquire::Select;

#[tokio::main]
async fn main() {
  if let Err(err) = entrypoint().await {
    println!("{}", err);
  }
}

async fn entrypoint() -> error::Result<()> {
  let args = Cli::parse();
  let curr_dir = std::env::current_dir().map_err(error::Error::GetCurrentDir)?;

  match args.command {
    Command::Init => {
      Lapse::init(curr_dir)?;
      println!("Initialized Lapse space");
    }
    Command::Ls => {
      let lapse = Lapse::open(curr_dir)?;
      let collection = lapse.get_request_collection(None);
      output_requests_collection(0, &collection);
    }
    Command::Send { request } => {
      let lapse = Lapse::open(curr_dir)?;

      let selected_request = match request {
        Some(existing) => existing,
        None => {
          let tree = lapse.get_request_collection(None);
          let flat_requests = get_requests_flatlist(tree);

          let select = Select::new("Select the request", flat_requests);
          select.prompt().map_err(error::Error::InvokePrompt)?
        }
      };

      let req = lapse.get_request_file(&selected_request)?;

      let response = lapse.request(&req, selected_request).await?;
      println!("{}", response.text);
    }
    Command::Completion { shell } => {
      let mut out = stdout();
      generate_completion(shell, &mut out);
    }
    Command::Env(env_command) => match env_command {
      cli::EnvCommand::Switch { name } => {
        let lapse = Lapse::open(curr_dir)?;
        lapse.switch_env(&name)?;
      }
    },
  }

  Ok(())
}

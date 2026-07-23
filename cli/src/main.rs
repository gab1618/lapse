mod cli;
mod collection;
mod completion;
mod error;
mod select;

pub use error::{Error, Result};

use std::io::stdout;

use clap::Parser;
use cli::Cli;

use lapse::{Lapse, tree::resource::Resource};

use crate::{
  cli::{Command, ScriptCommand},
  collection::output_tree,
  completion::generate_completion,
  select::select_tree_entry,
};

#[tokio::main]
async fn main() {
  if let Err(err) = entrypoint().await {
    eprintln!("{}", err);
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
      let collection = lapse.get_resource_tree(Resource::Requests, None)?;
      output_tree(0, &collection);
    }
    Command::Send { request } => {
      let lapse = Lapse::open(curr_dir)?;
      let tree = lapse.get_resource_tree(Resource::Requests, None)?;

      let selected_request = select_tree_entry(&tree, request)?;

      let response = lapse.request(selected_request).await?;
      println!("{}", response.text);
    }
    Command::Completion { shell } => {
      let mut out = stdout();
      generate_completion(shell, &mut out);
    }
    Command::Env(env_command) => {
      let lapse = Lapse::open(curr_dir)?;
      let tree = lapse.get_resource_tree(Resource::Env, None)?;

      match env_command {
        cli::EnvCommand::Switch { name } => {
          let seleced_env = select_tree_entry(&tree, name)?;
          lapse.switch_env(&seleced_env)?;
          println!("Switched to env: {}", seleced_env);
        }
        cli::EnvCommand::Ls => {
          // TODO: mark the current env you are in
          output_tree(0, &tree);
        }
      }
    }
    Command::Run { script } => {
      let lapse = Lapse::open(curr_dir)?;
      let tree = lapse.get_resource_tree(Resource::Scripts, None)?;

      let selected_script = select_tree_entry(&tree, script)?;

      lapse.run_script(&selected_script).await?;
    }
    Command::Script(script_command) => {
      let lapse = Lapse::open(curr_dir)?;
      let tree = lapse.get_resource_tree(Resource::Scripts, None)?;

      match script_command {
        ScriptCommand::Run { script } => {
          let selected_script = select_tree_entry(&tree, script)?;
          lapse.run_script(&selected_script).await?;
        }
        ScriptCommand::Ls => {
          output_tree(0, &tree);
        }
      }
    }
  }

  Ok(())
}

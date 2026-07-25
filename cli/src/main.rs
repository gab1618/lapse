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

pub fn open_lapse() -> Result<Lapse> {
  let curr_dir = std::env::current_dir().map_err(error::Error::GetCurrentDir)?;
  Ok(Lapse::open(curr_dir)?)
}

#[tokio::main]
async fn main() {
  if let Err(err) = entrypoint().await {
    eprintln!("{}", err);
  }
}

async fn entrypoint() -> error::Result<()> {
  let args = Cli::parse();

  match args.command {
    Command::Init => {
      let curr_dir = std::env::current_dir().map_err(error::Error::GetCurrentDir)?;
      Lapse::init(curr_dir)?;
      println!("Initialized Lapse space");
    }
    Command::Ls { path } => {
      let lapse = open_lapse()?;
      let collection = lapse.get_resource_tree(Resource::Requests, path)?;
      output_tree(0, &collection);
    }
    Command::Send { request } => {
      let lapse = open_lapse()?;
      let tree = lapse.get_resource_tree(Resource::Requests, None)?;

      let selected_request = select_tree_entry(&tree, request)?;

      let response = lapse
        .request(selected_request, lapse.get_eval_ctx()?)
        .await?;
      println!("{}", response.text);
    }
    Command::Completion { shell } => {
      let mut out = stdout();
      generate_completion(shell, &mut out);
    }
    Command::Env(env_command) => {
      let lapse = open_lapse()?;

      match env_command {
        cli::EnvCommand::Switch { name } => {
          let tree = lapse.get_resource_tree(Resource::Env, None)?;
          let seleced_env = select_tree_entry(&tree, name)?;

          lapse.switch_env(&seleced_env)?;
          println!("Switched to env: {}", seleced_env);
        }
        cli::EnvCommand::Ls { path } => {
          // TODO: mark the current env you are in
          let tree = lapse.get_resource_tree(Resource::Env, path)?;
          output_tree(0, &tree);
        }
      }
    }
    Command::Run { script } => {
      let lapse = open_lapse()?;
      let tree = lapse.get_resource_tree(Resource::Scripts, None)?;

      let selected_script = select_tree_entry(&tree, script)?;

      lapse.run_script(&selected_script).await?;
    }
    Command::Script(script_command) => {
      let lapse = open_lapse()?;

      match script_command {
        ScriptCommand::Run { script } => {
          let tree = lapse.get_resource_tree(Resource::Scripts, None)?;
          let selected_script = select_tree_entry(&tree, script)?;
          lapse.run_script(&selected_script).await?;
        }
        ScriptCommand::Ls { path } => {
          let tree = lapse.get_resource_tree(Resource::Scripts, path)?;
          output_tree(0, &tree);
        }
      }
    }
  }

  Ok(())
}

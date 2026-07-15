mod cli;
mod collection;
mod completion;

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
  let args = Cli::parse();
  let curr_dir = std::env::current_dir().expect("Somehow we don't have a current dir");

  match args.command {
    Command::Init => {
      Lapse::init(curr_dir).unwrap();
      println!("Initialized Lapse space");
    }
    Command::Ls => {
      let lapse = Lapse::open(curr_dir);
      let collection = lapse.get_request_collection(None);
      output_requests_collection(0, &collection);
    }
    Command::Send { request } => {
      let lapse = Lapse::open(curr_dir);

      let selected_request = match request {
        Some(existing) => existing,
        None => {
          let tree = lapse.get_request_collection(None);
          let flat_requests = get_requests_flatlist(tree);

          let select = Select::new("Select the request", flat_requests);
          select.prompt().unwrap()
        }
      };

      let req = lapse.get_request_file(&selected_request);

      let response = lapse.request(&req, selected_request).await;
      println!("{}", response.text);
    }
    Command::Completion { shell } => {
      let mut out = stdout();
      generate_completion(shell, &mut out);
    }
    Command::Env(env_command) => match env_command {
      cli::EnvCommand::Switch { name } => {
        let lapse = Lapse::open(curr_dir);
        lapse.switch_env(&name).unwrap();
      }
    },
  }
}

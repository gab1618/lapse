mod cli;
mod collection;

use clap::Parser;
use cli::Cli;

use lapse::Lapse;

use crate::{cli::Command, collection::output_requests_collection};

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
      let response = lapse.request(&request).await;

      let body = response.text().await.unwrap();
      println!("{}", body);
    }
  }
}

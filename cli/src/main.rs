mod cli;

use clap::Parser;
use cli::Cli;

use lapse::Lapse;

use crate::cli::Command;

fn main() {
  let args = Cli::parse();
  let curr_dir = std::env::current_dir().expect("Somehow we don't have a current dir");

  match args.command {
    Command::Init => {
      Lapse::init(curr_dir).unwrap();
      println!("Initialized Lapse space");
    }
  }
}

//! How to extract subcommands' args into external structs.
use crate::command::Command;
use structopt::StructOpt;

mod command;
mod completion;
mod export;
mod fetch;
mod git;
mod install;
mod print;
mod shapefile;
mod sqlite;
mod std;

#[derive(Debug, StructOpt)]
#[structopt(name = "wof", author, about)]
pub struct ApplicationArguments {
  #[structopt(subcommand)]
  pub command: Command,
}

fn main() {
  let opt = ApplicationArguments::from_args();

  opt.command.exec();
}

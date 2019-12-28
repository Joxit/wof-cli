//! How to extract subcommands' args into external structs.
use crate::command::Command;
use structopt::StructOpt;

mod command;
mod export;
mod git;
mod install;
mod shapefile;
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

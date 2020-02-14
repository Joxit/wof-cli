use crate::commands::Command;
use structopt::StructOpt;

mod commands;
mod git;
mod repo;
pub mod ser;
mod sqlite;
mod std;
pub mod utils;
mod wof;

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

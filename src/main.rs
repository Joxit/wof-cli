use crate::commands::Command;
use clap::{Args, CommandFactory, Parser};

#[macro_use]
extern crate lazy_static;

mod commands;
pub mod expression;
mod git;
mod repo;
pub use self::expression::*;
mod ser;
pub use self::ser::*;
mod de;
pub use self::de::*;
pub mod export;
mod fix;
mod postgres;
mod shapefile;
mod sqlite;
mod std;
pub mod types;
pub mod utils;
mod wof;
pub use self::wof::WOFGeoJSON;
pub use json::object::Object as JsonObject;
pub use json::JsonValue;

#[derive(Parser, Debug)]
#[structopt(name = "wof", author, version, about)]
pub struct Wof {
  #[command(subcommand)]
  pub command: Command,
}

impl Wof {
  pub fn display_help(cmd: &str) {
    let clap = Self::augment_args(Self::command());
    let args = format!("{} {} --help", clap, cmd);
    clap.get_matches_from(args.split(" "));
  }
}

fn main() {
  let opt = Wof::parse();

  opt.command.exec();
}

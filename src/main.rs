use crate::commands::Command;
use structopt::StructOpt;

mod commands;
mod git;
mod repo;
mod ser;
pub use self::ser::*;
mod de;
pub use self::de::*;
pub mod export;
mod shapefile;
mod sqlite;
mod std;
pub mod types;
pub mod utils;
mod wof;
pub use self::wof::WOFGeoJSON;
pub use json::object::Object as JsonObject;
pub use json::JsonValue;

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

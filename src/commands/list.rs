use crate::repo::Walk;
use crate::utils::ResultExit;
use std::io::Write;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct List {
  /// Paths to WOF documents.
  #[structopt(default_value = ".")]
  pub directories: Vec<String>,
  /// List also alternate geometries.
  #[structopt(long = "alt")]
  pub alt: bool,
  /// Don't print deprecated features.
  #[structopt(long = "no-deprecated")]
  pub no_deprecated: bool,
}

impl List {
  pub fn exec(&self) {
    for directory in &self.directories {
      for entry in Walk::new(directory.to_string(), self.alt, !self.no_deprecated) {
        if let Ok(path) = entry {
          writeln!(std::io::stdout(), "{}", path.path().display()).exit_silently();
        }
      }
    }
  }
}

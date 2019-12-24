use crate::export::Export;
use crate::shapefile::Shapefile;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Install {
  /// Name of the package to install (saved in ~/.wof directory)
  #[structopt(possible_values = &["export", "shapefile"])]
  pub package: String,
}

impl Install {
  pub fn exec(&self) {
    match self.package.as_ref() {
      "export" => Export::install(),
      "shapefile" => Shapefile::install(),
      _ => {
        eprintln!("Incorrect package to install.");
        std::process::exit(127)
      }
    }
  }
}

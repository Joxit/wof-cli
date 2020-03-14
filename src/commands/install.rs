use crate::commands::export::Export;
use crate::utils::ResultExit;
use log::error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Install {
  /// Name of the package to install (saved in ~/.wof directory)
  #[structopt(possible_values = &["export"])]
  pub package: String,
}

impl Install {
  pub fn exec(&self) {
    crate::utils::logger::set_verbose(false, "wof::install").expect_exit("Can't init logger.");
    match self.package.as_ref() {
      "export" => Export::install(),
      _ => {
        error!("Incorrect package to install.");
        std::process::exit(127)
      }
    }
  }
}

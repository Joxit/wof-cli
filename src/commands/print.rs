use crate::ser::{Generator, WOFGenerator};
use crate::std::ResultExit;
use crate::utils;
use json;
use std::io::Read;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Print {
  /// Ids to WOF documents to print
  pub ids: Vec<String>,
  /// Remove the geometry before print
  #[structopt(long = "no-geom")]
  pub no_geom: bool,
}

impl Print {
  pub fn exec(&self) {
    for id in &self.ids {
      let path = utils::id_to_data_path_geojson(id);
      if path.exists() && !path.is_dir() {
        let mut file =
          std::fs::File::open(path).expect_exit(format!("Can't open id {}", id).as_str());
        let message_error = format!("Something goes wrong when printing {}", id);
        if !self.no_geom {
          std::io::copy(&mut file, &mut std::io::stdout()).expect_exit(message_error.as_str());
        } else {
          let mut buffer = String::new();
          file
            .read_to_string(&mut buffer)
            .expect_exit(message_error.as_str());
          let mut obj = json::parse(&mut buffer).expect_exit(message_error.as_str());
          let _ = obj.remove("geometry");
          WOFGenerator::new(&mut std::io::stdout())
            .write_json(&obj)
            .expect_exit(message_error.as_str());
        }
      } else {
        eprintln!("Skipping {}, does not exists", id);
      }
    }
  }
}

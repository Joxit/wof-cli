use crate::repo::Walk;
use crate::sqlite;
use crate::utils::ResultExit;
use std::io::{Read, Write};
use std::path::Path;
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
  /// Print minified geojson instead of path.
  #[structopt(long = "print-geojson")]
  pub print_geojson: bool,
}

impl List {
  pub fn exec(&self) {
    for directory in &self.directories {
      if Path::new(directory).is_dir() {
        self.walk_directory(directory)
      } else {
        self.list_sqlite(directory)
      }
    }
  }

  pub fn walk_directory(&self, directory: &String) {
    for entry in Walk::new(directory.to_string(), self.alt, !self.no_deprecated) {
      if let Ok(path) = entry {
        if self.print_geojson {
          let mut file = std::fs::File::open(path.path()).exit_silently();
          let mut buffer = String::new();
          file.read_to_string(&mut buffer).exit_silently();
          let json = crate::parse_string_to_json(&buffer).exit_silently();
          crate::ser::json_to_writer(&json, &mut std::io::stdout()).exit_silently();
          writeln!(std::io::stdout(), "").exit_silently()
        } else {
          writeln!(std::io::stdout(), "{}", path.path().display()).exit_silently();
        }
      }
    }
  }

  pub fn list_sqlite(&self, directory: &String) {
    let sqlite = sqlite::SQLite::new(
      directory,
      sqlite::SQLiteOpts {
        pretty: false,
        deprecated: !self.no_deprecated,
        alt: self.alt,
        ..Default::default()
      },
    )
    .expect_exit("Can't open the database");

    if self.print_geojson {
      sqlite
        .write_all_geojsons(&mut std::io::stdout())
        .expect_exit("Can't write to stdout");
    } else {
      sqlite
        .write_all_ids(&mut std::io::stdout())
        .expect_exit("Can't write to stdout");
    }
  }
}

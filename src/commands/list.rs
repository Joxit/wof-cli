use crate::expression::{Evaluate, Predicate};
use crate::repo::Walk;
use crate::sqlite;
use crate::utils::ResultExit;
use clap::Parser;
use log::error;
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Parser)]
pub struct List {
  /// Paths to WOF documents.
  #[arg(default_value = ".")]
  pub directories: Vec<String>,
  /// List also alternate geometries.
  #[arg(long = "alt")]
  pub alt: bool,
  /// Don't print deprecated features.
  #[arg(long = "no-deprecated")]
  pub no_deprecated: bool,
  /// Print minified geojson instead of path.
  #[arg(long = "print-geojson")]
  pub print_geojson: bool,
  /// Filter lister geojson with expression.
  #[arg(long = "filter")]
  pub filter: Option<String>,
}

impl List {
  pub fn exec(&self) {
    let predicate: Predicate = if let Some(predicate) = &self.filter {
      if !self.print_geojson {
        error!("When --filter is used, you must also use --print-geojson");
        std::process::exit(1);
      }
      Predicate::try_from(predicate.clone()).expect_exit("Inccorect expression")
    } else {
      Predicate::Boolean(true)
    };
    for directory in &self.directories {
      if Path::new(directory).is_dir() {
        self.walk_directory(directory, &predicate)
      } else {
        self.list_sqlite(directory, &predicate)
      }
    }
  }

  pub fn walk_directory(&self, directory: &String, predicate: &Predicate) {
    for entry in Walk::new(directory.to_string(), self.alt, !self.no_deprecated) {
      if let Ok(path) = entry {
        if self.print_geojson {
          let mut file = std::fs::File::open(path.path()).exit_silently();
          let mut buffer = String::new();
          file.read_to_string(&mut buffer).exit_silently();
          let json = crate::parse_string_to_json(&buffer).exit_silently();
          if let Predicate::Boolean(true) = json
            .eval(&predicate)
            .expect_exit("Can't evaluate expression")
          {
            crate::ser::json_to_writer(&json, &mut std::io::stdout()).exit_silently();
            writeln!(std::io::stdout(), "").exit_silently()
          }
        } else {
          writeln!(std::io::stdout(), "{}", path.path().display()).exit_silently();
        }
      }
    }
  }

  pub fn list_sqlite(&self, directory: &String, predicate: &Predicate) {
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
        .write_all_geojsons(&mut std::io::stdout(), &predicate)
        .expect_exit("Can't write to stdout");
    } else {
      sqlite
        .write_all_ids(&mut std::io::stdout())
        .expect_exit("Can't write to stdout");
    }
  }
}

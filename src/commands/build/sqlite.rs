use crate::commands::assert_directory_exists;
use crate::repo::Walk;
use crate::sqlite;
use crate::utils::ResultExit;
use log::{error, info};
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct SQLite {
  /// WOF data directories
  #[structopt(default_value = ".")]
  pub directories: Vec<String>,
  /// Where to store the final build file. If empty the code will attempt to create whosonfirst-data-latest.db the current working directory.
  #[structopt(long = "out", default_value = "whosonfirst-data-latest.db")]
  pub out: String,
  /// Don't insert deprecated features.
  #[structopt(long = "no-deprecated")]
  pub no_deprecated: bool,
  /// Don't insert deprecated features.
  #[structopt(long = "no-pretty")]
  pub no_pretty: bool,
  /// Preset for pelias use. Will insert only in geojson and spr tables.
  #[structopt(long = "preset", possible_values = &["pelias"])]
  pub preset: Option<String>,
  /// Display timings during the build process, implies verbose.
  #[structopt(long = "timings")]
  pub timings: bool,
  /// Activate verbose mode.
  #[structopt(short = "v", long = "verbose")]
  pub verbose: bool,
}

impl SQLite {
  pub fn exec(&self) {
    let out_path = Path::new(&self.out).to_path_buf();
    crate::utils::logger::set_verbose(self.verbose, "wof::build::sqlite")
      .expect_exit("Can't init logger.");
    let parent = out_path
      .parent()
      .expect_exit("Can't create a folder for your database file. No parent directory.")
      .to_path_buf();

    assert_directory_exists(&parent);

    let pelias_preset = if let Some(preset) = &self.preset {
      info!("Using pelias preset, only geojson and spr tables will be filled.");
      *preset == String::from("pelias")
    } else {
      false
    };

    info!("Will create database: `{}`", out_path.as_path().display());
    let sqlite = sqlite::SQLite::new(
      out_path,
      sqlite::SQLiteOpts {
        pretty: !self.no_pretty,
        deprecated: !self.no_deprecated,
        names: !pelias_preset,
        ancestors: !pelias_preset,
        concordances: !pelias_preset,
        ..Default::default()
      },
    )
    .expect_exit("Can't open the database");

    info!("Will create tables.");
    sqlite.create_tables().expect_exit("Can't create tables");

    if crate::commands::input_pipe() {
      info!("Start import from stdin.");
      loop {
        let mut buffer = String::new();
        match std::io::stdin().read_line(&mut buffer) {
          Ok(0) => break,
          Ok(_) => {
            if let Err(e) = sqlite.add_string(buffer) {
              error!("Something goes wrong with an entry from stdin: {}", e);
            }
          }
          Err(_) => break,
        }
      }
    } else {
      for directory in &self.directories {
        info!("Start import for stdin `{}`", directory);
        for entry in Walk::new(directory.to_string(), false, true) {
          if let Ok(path) = entry {
            if let Err(e) = sqlite.add_file(path.path()) {
              error!("Something goes wrong for {}: {}", path.path().display(), e);
            }
          }
        }
      }
    }
    info!("Import finished successfully.");
  }
}

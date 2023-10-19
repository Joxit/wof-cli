use crate::commands::assert_directory_exists;
use crate::sqlite;
use crate::utils::ResultExit;
use clap::builder::PossibleValuesParser;
use clap::Parser;
use log::info;
use std::path::Path;

#[derive(Debug, Parser)]
pub struct SQLite {
  /// WOF data directories
  #[arg(default_value = ".")]
  pub directories: Vec<String>,
  /// Where to store the final build file. If empty the code will attempt to create whosonfirst-data-latest.db the current working directory.
  #[arg(long = "out", default_value = "whosonfirst-data-latest.db")]
  pub out: String,
  /// Don't insert deprecated features.
  #[arg(long = "no-deprecated")]
  pub no_deprecated: bool,
  /// Don't prettify the geojson.
  #[arg(long = "no-pretty")]
  pub no_pretty: bool,
  /// Preset for pelias use. Will insert only in geojson and spr tables.
  #[arg(long = "preset", value_parser = PossibleValuesParser::new(&["pelias"]))]
  pub preset: Option<String>,
  /// Display timings during the build process, implies verbose.
  #[arg(long = "timings")]
  pub timings: bool,
  /// Activate verbose mode.
  #[arg(short = 'v', long = "verbose")]
  pub verbose: bool,
}

impl SQLite {
  pub fn exec(&self) {
    let out_path = Path::new(&self.out).to_path_buf();
    crate::utils::logger::set_verbose(self.verbose || self.timings, "wof::build::sqlite")
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

    info!("Creating database: `{}`", out_path.as_path().display());
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

    info!("Creating tables and indexes.");
    sqlite.create_tables().expect_exit("Can't create tables");

    crate::commands::build::build_database(&self.directories, self.timings, &mut |buffer, file| {
      if let Some(buffer) = buffer {
        sqlite.add_string(buffer)
      } else if let Some(file) = file {
        sqlite.add_file(file)
      } else {
        Ok(())
      }
    });
  }
}

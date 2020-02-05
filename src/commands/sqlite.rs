use crate::commands::assert_directory_exists;
use crate::sqlite;
use crate::std::ResultExit;
use crate::walk::Walk;
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
    let parent = out_path
      .parent()
      .expect_exit("Can't create a folder for your database file. No parent directory.")
      .to_path_buf();

    assert_directory_exists(&parent);

    let sqlite = sqlite::SQLite::new(out_path).expect_exit("Can't open the database");
    sqlite.create_indexes().expect_exit("Can't create indexes");
    for directory in &self.directories {
      for entry in Walk::new(directory.to_string(), false, !self.no_deprecated) {
        if let Ok(path) = entry {
          if let Err(e) = sqlite.add_file(path.path()) {
            eprintln!("Something goes wrong for {}: {}", path.path().display(), e);
          }
        }
      }
    }
  }
}

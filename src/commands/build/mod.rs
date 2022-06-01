pub use crate::commands::build::postgres::Postgres;
pub use crate::commands::build::shapefile::Shapefile;
pub use crate::commands::build::sqlite::SQLite;
use crate::repo::Walk;
use log::{error, info};
use std::path::PathBuf;
use std::time::SystemTime;
use structopt::StructOpt;

mod postgres;
mod shapefile;
mod sqlite;

#[derive(Debug, StructOpt)]
pub enum Build {
  /// Who's On First documents to PostgreSQL database.
  #[structopt(name = "postgres")]
  Postgres(Postgres),
  /// Who's On First documents to ESRI shapefiles.
  #[structopt(name = "shapefile")]
  Shapefile(Shapefile),
  /// Who's On First documents to SQLite database.
  #[structopt(name = "sqlite")]
  SQLite(SQLite),
}

impl Build {
  pub fn exec(&self) {
    match self {
      Build::Postgres(executable) => executable.exec(),
      Build::Shapefile(executable) => executable.exec(),
      Build::SQLite(executable) => executable.exec(),
    }
  }
}

pub fn build_database<F: FnMut(Option<String>, Option<PathBuf>) -> Result<(), String>>(
  directories: &Vec<String>,
  timings: bool,
  add: &mut F,
) {
  let mut count = 0u64;
  let import_start = SystemTime::now();

  if crate::commands::input_pipe() {
    info!("Start import from stdin.");
    loop {
      let mut buffer = String::new();
      match std::io::stdin().read_line(&mut buffer) {
        Ok(0) => break,
        Ok(_) => {
          if let Err(e) = add(Some(buffer), None) {
            error!("Something goes wrong with an entry from stdin: {}", e);
          } else {
            count = count + 1;
          }
        }
        Err(_) => break,
      }
    }
  } else {
    for directory in directories {
      info!("Start import for directory `{}`", directory);
      let start = SystemTime::now();
      for entry in Walk::new(directory.to_string(), false, true) {
        if let Ok(path) = entry {
          let p = path.path();
          if let Err(e) = add(None, Some(p.to_path_buf())) {
            error!("Something goes wrong for {}: {}", p.display(), e);
          } else {
            count = count + 1;
          }
        }
      }
      if timings {
        info!(
          "Import for `{}` took {:?}.",
          directory,
          start.elapsed().unwrap()
        );
      }
    }
  }

  if timings {
    info!(
      "Imported {} documents successfully in {:?}.",
      count,
      import_start.elapsed().unwrap()
    );
  } else {
    info!("Imported {} documents successfully.", count);
  }
}

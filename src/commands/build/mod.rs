pub use crate::commands::build::shapefile::Shapefile;
pub use crate::commands::build::sqlite::SQLite;
use structopt::StructOpt;

mod shapefile;
mod sqlite;

#[derive(Debug, StructOpt)]
pub enum Build {
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
      Build::Shapefile(executable) => executable.exec(),
      Build::SQLite(executable) => executable.exec(),
    }
  }
}

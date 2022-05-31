use crate::std::StringifyError;
use crate::wof::WOFGeoJSON;
pub use postgres::Config;
use postgres::{Client, NoTls};
use std::path::Path;

mod statements;

const DEFAULT_SRID: i32 = 4326;

pub struct Postgres {
  /// Postgres client for requests
  client: Client,
  /// The used srid for stored geometries
  srid: i32,
}

impl Postgres {
  /// Create a connection to a database, will check if the selected srid is the same as the existing table.
  pub fn new(config: Config, srid: Option<i32>) -> Result<Self, String> {
    let mut client = config
      .connect(NoTls)
      .stringify_err("connection to database")?;
    let current_srid = Postgres::get_current_srid(&mut client)?;
    Postgres::check_srids(current_srid, srid)?;
    let srid = srid.unwrap_or(current_srid.unwrap_or(DEFAULT_SRID));
    Ok(Self { client, srid })
  }

  /// Create all tables, indexes and configure the database.
  pub fn create_tables(&mut self) -> Result<(), String> {
    self
      .client
      .execute(
        &statements::TABLE_GEOMETRIES.replace("${srid}", &self.srid.to_string()),
        &[],
      )
      .stringify_err("Can't create wof_geometries table")?;
    self
      .client
      .batch_execute(&statements::INDEXES_GEOMETRIES.replace("${srid}", &self.srid.to_string()))
      .stringify_err("Can't create wof_geometries indexes")?;
    Ok(())
  }

  /// Add a file to the database, the file must be a WOF GeoJSON.
  pub fn add_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
    let json = crate::parse_file_to_json(path.as_ref().to_path_buf())?;
    let geojson = WOFGeoJSON::as_valid_wof_geojson(&json)?;
    self.add(geojson)
  }

  /// Add the string content to the database, it must be a WOF GeoJSON.
  pub fn add_string(&mut self, buf: String) -> Result<(), String> {
    let json = crate::parse_string_to_json(buf)?;
    let geojson = WOFGeoJSON::as_valid_wof_geojson(&json)?;
    self.add(geojson)
  }

  /// Add a WOFGeoJSON document to the database.
  pub fn add(&mut self, document: WOFGeoJSON) -> Result<(), String> {
    self
      .client
      .execute(
        statements::INSERT_GEOMETRIES,
        &[
          &document.id,
          &document.geometry.dump(),
          &document.get_source(),
          &document.properties.dump(),
          &document.is_alt_geom(),
          &document.get_last_modified(),
          &self.srid,
        ],
      )
      .stringify_err(&format!("Can't insert document {}", document.id))?;
    Ok(())
  }

  fn get_current_srid(client: &mut Client) -> Result<Option<i32>, String> {
    if let Ok(row) = client.query_one(statements::GET_SRID, &[]) {
      Ok(Some(row.try_get(0).stringify_err(
        "Can't retrieve the SRID of wof_geometries with Find_SRID",
      )?))
    } else {
      Ok(None)
    }
  }

  fn check_srids(src: Option<i32>, dst: Option<i32>) -> Result<(), String> {
    if src.is_some() && dst.is_some() {
      let src = src.unwrap();
      let dst = dst.unwrap();
      if src != dst {
        return Err(format!("The table wof_geometries is using srid {} and you are requesting srid {}. Please drop the table first", src, dst));
      }
    }
    Ok(())
  }
}

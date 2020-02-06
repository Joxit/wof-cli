use crate::std::StringifyError;
use crate::wof::WOFGeoJSON;
use rusqlite::{params, Connection, Error as SQLiteError};
use std::path::{Path, PathBuf};
mod statements;

#[derive(Debug)]
pub struct SQLite {
  conn: Connection,
  opts: SQLiteOpts,
}

#[derive(Debug, Clone, Default)]
pub struct SQLiteOpts {
  pub pretty: bool,
}

impl SQLite {
  pub fn new(path: PathBuf, opts: SQLiteOpts) -> Result<Self, String> {
    Ok(SQLite {
      conn: Connection::open(path.as_path()).stringify_err("connection to database")?,
      opts: opts,
    })
  }

  pub fn create_indexes(&self) -> Result<(), String> {
    self
      .conn
      .execute_batch(statements::TABLE_GEOJSON)
      .stringify_err("geojson table")?;
    self
      .conn
      .execute_batch(statements::INDEXES_GEOJSON)
      .stringify_err("geojson indexes")?;
    self
      .conn
      .execute_batch(statements::TABLE_SPR)
      .stringify_err("spr table")?;
    self
      .conn
      .execute_batch(statements::INDEXES_SPR)
      .stringify_err("spr indexes")?;
    self
      .conn
      .execute_batch(statements::TABLE_NAMES)
      .stringify_err("names table")?;
    self
      .conn
      .execute_batch(statements::INDEXES_NAMES)
      .stringify_err("names indexes")?;
    self
      .conn
      .execute_batch(statements::TABLE_ANCESTORS)
      .stringify_err("ancestors table")?;
    self
      .conn
      .execute_batch(statements::INDEXES_ANCESTORS)
      .stringify_err("ancestors indexes")?;
    self
      .conn
      .execute_batch(statements::TABLE_CONCORDANCES)
      .stringify_err("concordances table")?;
    self
      .conn
      .execute_batch(statements::INDEXES_CONCORDANCES)
      .stringify_err("concordances indexes")?;
    Ok(())
  }

  pub fn add_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
    let json = WOFGeoJSON::parse_file_to_json(path.as_ref().to_path_buf())?;
    let geojson = WOFGeoJSON::as_valid_wof_geojson(&json)?;
    self.add(geojson)
  }

  pub fn add(&self, document: WOFGeoJSON) -> Result<(), String> {
    self
      .add_to_geojson(&document)
      .stringify_err("add document to geojson table")?;
    self
      .add_to_spr(&document)
      .stringify_err("add document to spr table")?;
    Ok(())
  }

  fn add_to_geojson(&self, doc: &WOFGeoJSON) -> Result<(), SQLiteError> {
    let mut input: Vec<u8> = Vec::new();
    if let Ok(_) = if self.opts.pretty {
      doc.pretty(&mut input)
    } else {
      doc.dump(&mut input)
    } {
      self.conn.execute(
        statements::INSERT_GEOJSON,
        params![
          doc.id,
          &input,
          doc.get_source(),
          doc.is_alt_geom(),
          doc.get_last_modified()
        ],
      )?;
      Ok(())
    } else {
      Err(SQLiteError::StatementChangedRows(0))
    }
  }

  fn add_to_spr(&self, doc: &WOFGeoJSON) -> Result<(), SQLiteError> {
    self.conn.execute(
      statements::INSERT_SPR,
      params![
        doc.id,
        doc.get_parent_id(),
        doc.get_name(),
        doc.get_placetype(),
        doc.get_country(),
        doc.get_repo(),
        doc.get_lat(),
        doc.get_lon(),
        doc.get_min_lat(),
        doc.get_min_lon(),
        doc.get_max_lat(),
        doc.get_max_lon(),
        bool_to_i32(doc.is_current()),
        bool_to_i32(doc.is_deprecated()),
        bool_to_i32(doc.is_ceased()),
        bool_to_i32(doc.is_superseded()),
        bool_to_i32(doc.is_superseding()),
        doc.get_superseded_by(),
        doc.get_supersedes(),
        doc.get_last_modified()
      ],
    )?;
    Ok(())
  }
}

fn bool_to_i32(b: bool) -> i32 {
  if b {
    1
  } else {
    0
  }
}

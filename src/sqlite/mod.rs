//! Module to create and add documents to WOF SQLites databases.
use crate::std::StringifyError;
use crate::wof::WOFGeoJSON;
use json::JsonValue;
use rusqlite::{params, Connection, Error as SQLiteError};
use std::io::Write;
use std::path::Path;
mod statements;

/// SQLite structure, own a connection to the database with options.
#[derive(Debug)]
pub struct SQLite {
  conn: Connection,
  opts: SQLiteOpts,
}

/// Options for the database, default values are the official configuration.
#[derive(Debug, Clone)]
pub struct SQLiteOpts {
  /// If true, will prettify the document in the geojson table.
  pub pretty: bool,
  /// If true, will also process deprecated documents.
  pub deprecated: bool,
  /// If true, will add documents in geojson table.
  pub geojson: bool,
  /// If true, will add documents in spr table.
  pub spr: bool,
  /// If true, will add documents in names table.
  pub names: bool,
  /// If true, will add documents in ancestors table.
  pub ancestors: bool,
  /// If true, will add documents in concordances table.
  pub concordances: bool,
  /// If true, will add alternative geometries in geojson table.
  pub alt: bool,
}

impl SQLite {
  /// Create a connection to a database, the parent folder should exists.
  pub fn new<P: AsRef<Path>>(path: P, opts: SQLiteOpts) -> Result<Self, String> {
    Ok(SQLite {
      conn: Connection::open(path).stringify_err("connection to database")?,
      opts: opts,
    })
  }

  /// Create all tables, indexes and configure the database.
  pub fn create_tables(&self) -> Result<(), String> {
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
    self
      .conn
      .execute_batch(statements::PRAGMA)
      .stringify_err("pragma statements")?;
    Ok(())
  }

  /// Add a file to the database, the file must be a WOF GeoJSON.
  pub fn add_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
    let json = crate::parse_file_to_json(path.as_ref().to_path_buf())?;
    let geojson = WOFGeoJSON::as_valid_wof_geojson(&json)?;
    self.add(geojson)
  }

  /// Add the string content to the database, it must be a WOF GeoJSON.
  pub fn add_string(&self, buf: String) -> Result<(), String> {
    let json = crate::parse_string_to_json(&buf)?;
    let geojson = WOFGeoJSON::as_valid_wof_geojson(&json)?;
    self.add(geojson)
  }

  /// Add a WOFGeoJSON document to the database.
  /// The `SQLiteOpts` is used here and it will define in which table the document should be added.
  pub fn add(&self, document: WOFGeoJSON) -> Result<(), String> {
    if !self.opts.deprecated && document.is_doc_deprecated() {
      return Ok(());
    }
    if self.opts.geojson {
      self
        .add_to_geojson(&document)
        .stringify_err("add document to geojson table")?;
    }
    if self.opts.spr {
      self
        .add_to_spr(&document)
        .stringify_err("add document to spr table")?;
    }
    if self.opts.names {
      self
        .add_to_names(&document)
        .stringify_err("add document to names table")?;
    }
    if self.opts.ancestors {
      self
        .add_to_ancestors(&document)
        .stringify_err("add document to ancestors table")?;
    }
    if self.opts.concordances {
      self
        .add_to_concordances(&document)
        .stringify_err("add document to ancestors table")?;
    }
    Ok(())
  }

  fn add_to_geojson(&self, doc: &WOFGeoJSON) -> Result<(), SQLiteError> {
    if doc.is_alt_geom() && !self.opts.alt {
      return Ok(());
    }

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

  fn add_to_names(&self, doc: &WOFGeoJSON) -> Result<(), SQLiteError> {
    for name in doc.get_names() {
      self.conn.execute(
        statements::INSERT_NAMES,
        params![
          doc.id,
          doc.get_placetype(),
          doc.get_country(),
          name.lang,
          name.extlang,
          "", // script
          "", // region
          "", // variant
          "", // extension
          name.variant,
          name.value,
          doc.get_last_modified()
        ],
      )?;
    }
    Ok(())
  }

  fn add_to_ancestors(&self, doc: &WOFGeoJSON) -> Result<(), SQLiteError> {
    for (ancestor_id, ancestor_placetype) in doc.get_ancestors() {
      self.conn.execute(
        statements::INSERT_ANCESTORS,
        params![
          doc.id,
          ancestor_id,
          ancestor_placetype,
          doc.get_last_modified()
        ],
      )?;
    }
    Ok(())
  }

  fn add_to_concordances(&self, doc: &WOFGeoJSON) -> Result<(), SQLiteError> {
    for (concordance_id, concordance_source) in doc.get_concordances() {
      self.conn.execute(
        statements::INSERT_CONCORDANCES,
        params![
          doc.id,
          concordance_id,
          concordance_source,
          doc.get_last_modified()
        ],
      )?;
    }
    Ok(())
  }

  pub fn write_all_ids<W: Write>(&self, mut writer: &mut W) -> Result<(), String> {
    let sql = if !self.opts.alt && !self.opts.deprecated {
      statements::SELECT_ALL_IDS_WITHOUT_ALT_AND_DEPRECATED
    } else if !self.opts.alt {
      statements::SELECT_ALL_IDS_WITHOUT_ALT
    } else if !self.opts.deprecated {
      statements::SELECT_ALL_IDS_WITHOUT_DEPRECATED
    } else {
      statements::SELECT_ALL_IDS
    };
    let mut stmt = self
      .conn
      .prepare(sql)
      .stringify_err("Can't get table geojson")?;

    let rows = stmt
      .query_map([], |row| {
        let res: i64 = row.get(0).unwrap();
        Ok(res)
      })
      .stringify_err("Can't get rows of table geojson")?;

    for id in rows {
      writeln!(&mut writer, "{}", id.unwrap()).stringify_err("Can't write to output")?;
    }
    Ok(())
  }

  pub fn write_all_geojsons<W: Write>(&self, mut writer: &mut W) -> Result<(), String> {
    let sql = if !self.opts.alt && !self.opts.deprecated {
      statements::SELECT_ALL_GEOJSONS_WITHOUT_ALT_AND_DEPRECATED
    } else if !self.opts.alt {
      statements::SELECT_ALL_GEOJSONS_WITHOUT_ALT
    } else if !self.opts.deprecated {
      statements::SELECT_ALL_GEOJSONS_WITHOUT_DEPRECATED
    } else {
      statements::SELECT_ALL_GEOJSONS
    };
    let mut stmt = self
      .conn
      .prepare(sql)
      .stringify_err("Can't get table geojson")?;

    let rows = stmt
      .query_map([], |row| {
        let res: Vec<u8> = row.get(0).unwrap();
        Ok(res)
      })
      .stringify_err("Can't get rows of table geojson")?;
    for body in rows {
      let body = std::str::from_utf8(&body.unwrap()).unwrap().to_string();
      let json = crate::parse_string_to_json(&body).stringify_err("Can't parse geojson body")?;
      crate::ser::json_to_writer(&json, &mut writer).stringify_err("Can't write to output")?;
      writeln!(&mut writer, "").stringify_err("Can't write to output")?;
    }
    Ok(())
  }

  pub fn get_geojson_by_id(&self, id: i64) -> Result<Option<JsonValue>, String> {
    let mut stmt = self
      .conn
      .prepare(statements::SELECT_GEOJSON_BY_ID)
      .stringify_err("Can't get table geojson")?;

    let mut rows = stmt
      .query_map(params![id], |row| {
        let res: Vec<u8> = row.get(0).unwrap();
        Ok(res)
      })
      .stringify_err("Can't get rows of table geojson")?;

    if let Some(body) = rows.next() {
      let body = std::str::from_utf8(&body.unwrap()).unwrap().to_string();
      let json = crate::parse_string_to_json(&body).stringify_err("Can't parse geojson body")?;
      Ok(Some(json))
    } else {
      Ok(None)
    }
  }

  pub fn set_geojson_alt(&self, id: i32, source: &String, is_alt: i64) -> Result<(), String> {
    self
      .conn
      .execute(statements::UPDATE_GEOJSON_ALT, params![is_alt, id, source])
      .stringify_err("Can't update table geojson")?;
    Ok(())
  }
}

impl Default for SQLiteOpts {
  fn default() -> Self {
    SQLiteOpts {
      pretty: true,
      deprecated: true,
      geojson: true,
      spr: true,
      names: true,
      ancestors: true,
      concordances: true,
      alt: true,
    }
  }
}

fn bool_to_i32(b: bool) -> i32 {
  if b {
    1
  } else {
    0
  }
}

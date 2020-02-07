use crate::ser::{DefaultGenerator, Generator, WOFGenerator};
use crate::std::StringifyError;
use crate::utils::JsonUtils;
pub use json::object::Object;
pub use json::{self, JsonValue};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct WOFGeoJSON<'a> {
  json: &'a Object,
  pub id: i32,
  pub r#type: String,
  pub properties: &'a Object,
  pub bbox: Vec<f64>,
  pub geometry: &'a Object,
}

impl<'a> WOFGeoJSON<'a> {
  pub fn parse_file_to_string(path: PathBuf) -> Result<String, String> {
    if !path.exists() {
      return Err(format!("File {} does not exists", path.as_path().display()));
    }

    let mut file = match std::fs::File::open(path) {
      Ok(file) => file,
      Err(e) => return Err(format!("{}", e)),
    };

    let mut buffer = String::new();
    if let Err(e) = file.read_to_string(&mut buffer) {
      return Err(format!("{}", e));
    };

    Ok(buffer)
  }

  pub fn parse_file_to_json(path: PathBuf) -> Result<JsonValue, String> {
    let buffer = WOFGeoJSON::parse_file_to_string(path)?;
    match json::parse(&buffer) {
      Ok(json) => Ok(json),
      Err(e) => return Err(format!("{}", e)),
    }
  }

  pub fn parse_string_to_json(buffer: String) -> Result<JsonValue, String> {
    match json::parse(&buffer) {
      Ok(json) => Ok(json),
      Err(e) => return Err(format!("{}", e)),
    }
  }

  pub fn as_valid_wof_geojson(json: &'a JsonValue) -> Result<Self, String> {
    json.assert_is_object()?;
    let json = json.as_object().unwrap();
    let props = if let Some(props) = json.get("properties") {
      props.assert_is_object().stringify_err("properties")?;
      props.as_object().unwrap()
    } else {
      return Err("properties not found in this geojson".to_string());
    };

    let bbox = if let Some(bbox) = json.get("bbox") {
      bbox.assert_is_array().stringify_err("bbox")?;
      let mut _bbox: Vec<f64> = vec![];
      for coord in bbox.as_array().unwrap() {
        _bbox.push(coord.as_f64().unwrap_or(0.0));
      }
      _bbox
    } else {
      return Err("bbox not found in this geojson".to_string());
    };

    let geom = if let Some(geom) = json.get("geometry") {
      geom.assert_is_object().stringify_err("geometry")?;
      geom.as_object().unwrap()
    } else {
      return Err("geometry not found in this geojson".to_string());
    };

    let id = if let Some(id) = json.get("id") {
      id.assert_is_number().stringify_err("id")?;
      id.as_i32().unwrap()
    } else {
      return Err("id not found in this geojson".to_string());
    };

    let r#type = if let Some(r#type) = json.get("type") {
      r#type.assert_is_string().stringify_err("type")?;
      r#type.as_str().unwrap()
    } else {
      return Err("type not found in this geojson".to_string());
    };

    Ok(WOFGeoJSON {
      json,
      id,
      r#type: r#type.to_string(),
      properties: props,
      bbox: bbox,
      geometry: geom,
    })
  }

  pub fn pretty(&self, mut writer: &mut dyn Write) -> Result<(), std::io::Error> {
    WOFGenerator::new(&mut writer).write_object(self.json)
  }

  pub fn dump(&self, mut writer: &mut dyn Write) -> Result<(), std::io::Error> {
    DefaultGenerator::new(&mut writer).write_object(self.json)
  }

  fn is_property_deprecated(&self, prop: &'static str) -> bool {
    match self.properties.get(prop) {
      Some(JsonValue::String(ref s)) => s != "uuuu",
      Some(JsonValue::Boolean(ref b)) => *b,
      Some(JsonValue::Array(ref a)) => a.len() > 0,
      _ => false,
    }
  }

  fn get_as_i32_or_else(&self, prop: &'static str, or_else: i32) -> i32 {
    match self.properties.get(prop) {
      Some(o) => o.as_i32().unwrap_or(or_else),
      _ => or_else,
    }
  }

  fn get_as_f64_or_else(&self, prop: &'static str, or_else: f64) -> f64 {
    match self.properties.get(prop) {
      Some(o) => o.as_f64().unwrap_or(or_else),
      _ => or_else,
    }
  }

  fn get_as_string_or_else(&self, prop: &'static str, or_else: &'static str) -> String {
    match self.properties.get(prop) {
      Some(JsonValue::String(s)) => s.to_string(),
      _ => or_else.to_string(),
    }
  }

  pub fn is_doc_deprecated(&self) -> bool {
    self.is_deprecated() || self.is_superseded() || !self.is_current()
  }

  pub fn is_alt_geom(&self) -> bool {
    match self.properties.get("src:alt_label") {
      Some(JsonValue::String(_)) => true,
      _ => false,
    }
  }

  pub fn is_current(&self) -> bool {
    self.get_as_i32_or_else("lastmodified", -1) != 0
  }

  pub fn is_deprecated(&self) -> bool {
    self.is_property_deprecated("edtf:deprecated")
  }

  pub fn is_ceased(&self) -> bool {
    self.is_property_deprecated("wof:cessation")
  }

  pub fn is_superseded(&self) -> bool {
    self.is_property_deprecated("wof:superseded_by")
  }

  pub fn is_superseding(&self) -> bool {
    self.is_property_deprecated("wof:supersedes")
  }

  pub fn get_source(&self) -> String {
    self.get_as_string_or_else("src:geom", "unknown")
  }

  pub fn get_last_modified(&self) -> i32 {
    self.get_as_i32_or_else("lastmodified", -1)
  }

  pub fn get_parent_id(&self) -> i32 {
    self.get_as_i32_or_else("wof:parent_id", -1)
  }

  pub fn get_placetype(&self) -> String {
    self.get_as_string_or_else("wof:placetype", "")
  }

  pub fn get_name(&self) -> String {
    self.get_as_string_or_else("wof:name", "")
  }

  pub fn get_country(&self) -> String {
    self.get_as_string_or_else("wof:country", "")
  }

  pub fn get_repo(&self) -> String {
    self.get_as_string_or_else("wof:repo", "")
  }

  pub fn get_lat(&self) -> f64 {
    let lat = self.get_as_f64_or_else("wof:latitude", 0.0);
    if lat != 0.0 {
      return lat;
    }
    (self.get_min_lat() + ((self.get_max_lat() - self.get_min_lat()) / 2.0))
  }

  pub fn get_lon(&self) -> f64 {
    let lat = self.get_as_f64_or_else("wof:longitude", 0.0);
    if lat != 0.0 {
      return lat;
    }
    (self.get_min_lon() + ((self.get_max_lon() - self.get_min_lon()) / 2.0))
  }

  pub fn get_min_lat(&self) -> f64 {
    self.bbox[1]
  }

  pub fn get_min_lon(&self) -> f64 {
    self.bbox[0]
  }

  pub fn get_max_lat(&self) -> f64 {
    self.bbox[3]
  }

  pub fn get_max_lon(&self) -> f64 {
    self.bbox[2]
  }

  pub fn get_superseded_by(&self) -> String {
    "".to_string()
  }

  pub fn get_supersedes(&self) -> String {
    "".to_string()
  }
}

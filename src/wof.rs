use crate::ser::{Generator, WOFGenerator};
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
  pub bbox: &'a Vec<JsonValue>,
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
      bbox.as_array().unwrap()
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
      bbox,
      geometry: geom,
    })
  }

  pub fn pretty(&self, mut writer: &mut dyn Write) -> Result<(), std::io::Error> {
    WOFGenerator::new(&mut writer).write_object(self.json)
  }

  fn is_property_deprecated(&self, prop: &'static str) -> bool {
    match self.properties.get(prop) {
      Some(JsonValue::String(ref s)) => s != "uuuu",
      Some(JsonValue::Boolean(ref b)) => *b,
      Some(JsonValue::Array(ref a)) => a.len() > 0,
      _ => false,
    }
  }

  pub fn is_deprecated(&self) -> bool {
    self.is_property_deprecated("edtf:deprecated")
      || self.is_property_deprecated("wof:superseded_by")
      || self.is_property_deprecated("mz:is_current")
  }
}

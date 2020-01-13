use crate::ser::{Generator, WOFGenerator};
use crate::utils::JsonUtils;
pub use json::object::Object;
pub use json::JsonValue;
use std::io::Write;

pub struct WOFGeoJSON<'a> {
  json: &'a Object,
  pub id: i32,
  pub r#type: String,
  pub properties: &'a Object,
  pub bbox: &'a Vec<JsonValue>,
  pub geometry: &'a Object,
}

impl<'a> WOFGeoJSON<'a> {
  pub fn as_valid_wof_geojson(json: &JsonValue) -> Result<WOFGeoJSON, String> {
    json.assert_is_object()?;
    let json = json.as_object().unwrap();
    let props = if let Some(props) = json.get("properties") {
      props
        .assert_is_object()
        .map_err(add_property_to_err("properties"))?;
      props.as_object().unwrap()
    } else {
      return Err("properties not found in this geojson".to_string());
    };
    let bbox = if let Some(bbox) = json.get("bbox") {
      bbox
        .assert_is_array()
        .map_err(add_property_to_err("bbox"))?;
      bbox.as_array().unwrap()
    } else {
      return Err("bbox not found in this geojson".to_string());
    };
    let geom = if let Some(geom) = json.get("geometry") {
      geom
        .assert_is_object()
        .map_err(add_property_to_err("geometry"))?;
      geom.as_object().unwrap()
    } else {
      return Err("geometry not found in this geojson".to_string());
    };
    let id = if let Some(id) = json.get("id") {
      id.assert_is_number().map_err(add_property_to_err("id"))?;
      id.as_i32().unwrap()
    } else {
      return Err("id not found in this geojson".to_string());
    };
    let r#type = if let Some(r#type) = json.get("type") {
      r#type
        .assert_is_string()
        .map_err(add_property_to_err("type"))?;
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
}

fn add_property_to_err(id: &'static str) -> impl Fn(String) -> String {
  move |err: String| format!("{}: {}", &id, &err)
}

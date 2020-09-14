use crate::std::StringifyError;
use crate::utils::JsonUtils;
use crate::{object_to_writer, object_to_writer_pretty, JsonObject, JsonValue};
use regex::Regex;
use std::io::Write;

/// Representation of a WOF GeoJSON, contains all required properties.
#[derive(Debug, Clone)]
pub struct WOFGeoJSON<'a> {
  json: &'a JsonObject,
  /// This is the id of the document.
  pub id: i32,
  /// This is the type of the document, should be `Feature`.
  pub r#type: String,
  /// All properties of the document, contains names, hierarchy...
  pub properties: &'a JsonObject,
  /// The BBox of the document.
  pub bbox: Vec<f64>,
  /// The raw Geometry, it's an inner GeoJSON. Types are `Point`, `MultiPoint`, `LineString`, `MultiLineString`, `Polygon` and `MultiPolygon`.
  pub geometry: &'a JsonObject,
}

#[derive(Debug, Clone)]
pub struct WofName<'a> {
  pub lang: &'a str,
  pub extlang: Option<&'a str>,
  pub variant: &'a str,
  pub value: &'a str,
}

impl<'a> WOFGeoJSON<'a> {
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
    object_to_writer_pretty(self.json, &mut writer)
  }

  pub fn dump(&self, mut writer: &mut dyn Write) -> Result<(), std::io::Error> {
    object_to_writer(self.json, &mut writer)
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

  fn get_as_string_or_else<'s>(&self, prop: &'s str, or_else: &'s str) -> String {
    match self.properties.get(prop) {
      Some(JsonValue::Short(s)) => s.to_string(),
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
    self.get_as_i32_or_else(
      "lastmodified",
      self.get_as_i32_or_else("wof:lastmodified", -1),
    )
  }

  pub fn get_parent_id(&self) -> i32 {
    self.get_as_i32_or_else("wof:parent_id", -1)
  }

  pub fn get_placetype(&self) -> String {
    self.get_as_string_or_else(
      "placetype",
      self.get_as_string_or_else("wof:placetype", "").as_str(),
    )
  }

  pub fn get_name(&self) -> String {
    self.get_as_string_or_else("wof:name", self.get_as_string_or_else("name", "").as_str())
  }

  pub fn get_names(&self) -> Vec<WofName> {
    let mut names: Vec<WofName> = vec![];
    let regex = Regex::new(
      r"name:(?P<lang>[a-zA-Z]{3})_((?P<extlang>[a-zA-Z_]+)_)?(?P<variant>x_[a-zA-Z_]+)$",
    )
    .unwrap();
    for (k, wof_names) in self.properties.iter() {
      if let Some(cap) = regex.captures(k) {
        let wof_names = wof_names.as_array();
        if !cap.name("lang").is_some() || !cap.name("variant").is_some() || !wof_names.is_some() {
          continue;
        }
        let wof_names = wof_names.unwrap();
        for wof_name in wof_names {
          if let Some(wof_name) = wof_name.as_str() {
            names.push(WofName {
              lang: cap.name("lang").unwrap().as_str(),
              extlang: cap.name("extlang").map(|e| e.as_str()),
              variant: cap.name("variant").unwrap().as_str(),
              value: wof_name,
            });
          }
        }
      }
    }
    names
  }

  pub fn get_country(&self) -> String {
    self.get_as_string_or_else("wof:country", "")
  }

  pub fn get_ancestors(&self) -> Vec<(i32, String)> {
    let mut ancestors: Vec<(i32, String)> = vec![];
    let regex = Regex::new(r"(?P<placetype>[a-zA-Z]*)_id$").unwrap();
    if let Some(wof_hierarchy) = self.properties.get("wof:hierarchy") {
      if let Some(hierarchies) = wof_hierarchy.as_array() {
        for hierarchy in hierarchies {
          let hierarchy = hierarchy.as_object();
          if !hierarchy.is_some() {
            continue;
          }
          for (placetype, id) in hierarchy.unwrap().iter() {
            if let Some(cap) = regex.captures(placetype) {
              if !cap.name("placetype").is_some() || !id.as_i32().is_some() {
                continue;
              }
              ancestors.push((
                id.as_i32().unwrap(),
                cap.name("placetype").unwrap().as_str().to_string(),
              ));
            }
          }
        }
      }
    }
    ancestors
  }

  pub fn get_concordances(&self) -> Vec<(i32, String)> {
    let mut concordances: Vec<(i32, String)> = vec![];
    if let Some(wof_concordances) = self.properties.get("wof:concordances") {
      if let Some(wof_concordances) = wof_concordances.as_object() {
        for (source, id) in wof_concordances.iter() {
          if !id.as_i32().is_some() {
            continue;
          }
          concordances.push((id.as_i32().unwrap(), source.to_string()));
        }
      }
    }
    concordances
  }

  pub fn get_repo(&self) -> String {
    self.get_as_string_or_else("wof:repo", "")
  }

  pub fn get_lat(&self) -> f64 {
    let lat = self.get_as_f64_or_else("wof:latitude", 0.0);
    if lat != 0.0 {
      return lat;
    }
    self.get_min_lat() + ((self.get_max_lat() - self.get_min_lat()) / 2.0)
  }

  pub fn get_lon(&self) -> f64 {
    let lat = self.get_as_f64_or_else("wof:longitude", 0.0);
    if lat != 0.0 {
      return lat;
    }
    self.get_min_lon() + ((self.get_max_lon() - self.get_min_lon()) / 2.0)
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

  pub fn get_belongs_to(&self) -> Vec<i32> {
    let mut belongs_to: Vec<i32> = vec![];
    if let Some(wof_belongs_to) = self.properties.get("wof:belongsto") {
      if let Some(wof_belongs_to) = wof_belongs_to.as_array() {
        for id in wof_belongs_to.iter() {
          if !id.as_i32().is_some() {
            continue;
          }
          belongs_to.push(id.as_i32().unwrap());
        }
      }
    }
    belongs_to
  }
}

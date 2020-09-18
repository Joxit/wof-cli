use crate::utils::{GeoCompute, JsonUtils};
use json::JsonValue;

pub fn export_json_value(json: JsonValue) -> Result<JsonValue, String> {
  json.assert_is_object()?;
  let geometry = export_geometry(&json)?;
  let properties = export_porperties(&json)?;
  let id = export_id(&json)?;
  let bbox = export_bbox(&json)?;

  Ok(json::object! {
    "id" => id,
    "type" => "Feature",
    "properties" => properties,
    "geometry" => geometry,
    "bbox" => bbox
  })
}

fn export_porperties(json: &JsonValue) -> Result<JsonValue, String> {
  let mut properties = JsonValue::new_object();

  let obj = json
    .as_object()
    .unwrap()
    .get("properties")
    .ok_or(String::from(
      "For WOF objects, `properties` key is required as an object.",
    ))?;

  Ok(properties)
}

fn export_geometry(json: &JsonValue) -> Result<JsonValue, String> {
  let mut geometry = JsonValue::new_object();

  let obj = json
    .as_object()
    .unwrap()
    .get("geometry")
    .ok_or(String::from(
      "For WOF objects, `geometry` key is required as an object.",
    ))?;

  Ok(geometry)
}

fn export_id(json: &JsonValue) -> Result<i64, String> {
  if let Some(id) = json.as_object().unwrap().get("id") {
    if id.is_number() {
      return Ok(id.as_i64().unwrap());
    }
  }

  let properties = json.as_object().unwrap().get("properties").unwrap();
  if let Some(id) = properties.as_object().unwrap().get("wof:id") {
    if id.is_number() {
      return Ok(id.as_i64().unwrap());
    }
  }

  Err(String::from("No id found for this feature."))
}

fn export_bbox(json: &JsonValue) -> Result<Vec<f64>, String> {
  let geom = json.as_object().unwrap().get("geometry").unwrap();
  Ok(geom.as_object().unwrap().compute_bbox())
}

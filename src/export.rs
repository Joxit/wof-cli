use crate::utils::{GeoCompute, GeoJsonUtils, JsonUtils};
use json::{array, JsonValue};

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
  let obj = json
    .as_object()
    .unwrap()
    .get("properties")
    .ok_or("`properties` key is required.")?;

  let mut properties = obj
    .as_object()
    .ok_or("`properties` key must be an object")?
    .clone();

  if properties.get("edtf:cessation").is_none() {
    properties.insert("edtf:cessation", JsonValue::from("uuuu"));
  }
  if properties.get("edtf:inception").is_none() {
    properties.insert("edtf:inception", JsonValue::from("uuuu"));
  }
  if properties.get("iso:country").is_none() {
    properties.insert("iso:country", JsonValue::from(""));
  }
  if properties.get("mz:hierarchy_label").is_none() {
    properties.insert("mz:hierarchy_label", JsonValue::from(1));
  }
  if properties.get("mz:is_current").is_none() {
    properties.insert("mz:is_current", JsonValue::from(-1));
  }
  if properties.get("src:geom").is_none() {
    properties.insert("src:geom", JsonValue::from("unknown"));
  }
  if properties.get("wof:belongsto").is_none() {
    properties.insert("wof:belongsto", array![]);
  }
  if properties.get("wof:breaches").is_none() {
    properties.insert("wof:breaches", array![]);
  }
  if properties.get("wof:country").is_none() {
    properties.insert("wof:country", JsonValue::from(""));
  }
  if properties.get("wof:hierarchy").is_none() {
    properties.insert("wof:hierarchy", array![]);
  }
  if properties.get("wof:parent_id").is_none() {
    properties.insert("wof:parent_id", JsonValue::from(-1));
  }
  if properties.get("wof:superseded_by").is_none() {
    properties.insert("wof:superseded_by", array![]);
  }
  if properties.get("wof:supersedes").is_none() {
    properties.insert("wof:supersedes", array![]);
  }
  if properties.get("wof:tags").is_none() {
    properties.insert("wof:tags", array![]);
  }

  Ok(JsonValue::Object(properties))
}

fn export_geometry(json: &JsonValue) -> Result<JsonValue, String> {
  let obj = json
    .as_object()
    .unwrap()
    .get("geometry")
    .ok_or("`geometry` key is required.")?
    .as_object()
    .ok_or("`geometry` key must be an object.")?;

  let coordinates = obj
    .get("coordinates")
    .ok_or("`coordinates` key is required.")?;
  let _type = obj.get("type").ok_or("In `geometry`, `type` is required")?;

  if !coordinates.is_array() {
    return Err(format!("`coordinates` key must be an array."));
  }

  let coords = match _type.as_str() {
    Some("Point") => json::array!(coordinates
      .as_geom_point()
      .ok_or("`coordinates` malformed for `type` Point.")?),
    Some("MultiPoint") => json::array!(coordinates
      .as_geom_multi_point()
      .ok_or("`coordinates` malformed for `type` MultiPoint.")?),
    Some("LineString") => json::array!(coordinates
      .as_geom_line()
      .ok_or("`coordinates` malformed for `type` LineString.")?),
    Some("MultiLineString") => json::array!(coordinates
      .as_geom_multi_line()
      .ok_or("`coordinates` malformed for `type` MultiLineString.")?),
    Some("Polygon") => json::array!(coordinates
      .as_geom_polygon()
      .ok_or("`coordinates` malformed for `type` Polygon.")?),
    Some("MultiPolygon") => json::array!(coordinates
      .as_geom_multi_polygon()
      .ok_or("`coordinates` malformed for `type` MultiPolygon.")?),
    Some(t) => return Err(format!("type {} is not supported.", t)),
    None => return Err(format!("In `geometry`, `type` key is required.")),
  };

  Ok(json::object! {
    "type" => _type.as_str().unwrap(),
    "coordinates" => coords
  })
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

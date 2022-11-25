use super::Predicate;
use crate::utils::JsonUtils;
use crate::wof::WOFGeoJSON;
use crate::JsonValue;

pub trait Evaluate {
  fn eval(&self, predicate: &Predicate) -> Result<Predicate, String> {
    match predicate {
      Predicate::And(left, right) => Ok(Predicate::Boolean(
        self.eval(&left)?.as_bool()? && self.eval(&right)?.as_bool()?,
      )),
      Predicate::Or(left, right) => Ok(Predicate::Boolean(
        self.eval(&left)?.as_bool()? || self.eval(&right)?.as_bool()?,
      )),
      Predicate::Eq(left, right) => Ok(Predicate::Boolean(self.eval(&left)? == self.eval(&right)?)),
      Predicate::Not(predicate) => Ok(Predicate::Boolean(!(self.eval(&predicate)?.as_bool()?))),
      Predicate::Boolean(b) => Ok(Predicate::Boolean(b == &true)),
      Predicate::Variable(s) => self.get_variable_value(s),
      _ => Ok(predicate.clone()),
    }
  }

  fn get_variable_value(&self, key: &String) -> Result<Predicate, String> {
    match key.as_str() {
      "geom_type" => self.get_geometry_type(),
      key => self.get_property(key),
    }
  }
  fn get_geometry_type(&self) -> Result<Predicate, String>;
  fn get_property(&self, key: &str) -> Result<Predicate, String>;
}

impl Evaluate for WOFGeoJSON<'_> {
  fn get_geometry_type(&self) -> Result<Predicate, String> {
    self
      .geometry
      .get("type")
      .ok_or(format!("Evaluated json must contains geometry.type"))
      .map(|value| {
        value
          .as_str()
          .map(|value| Predicate::String(value.to_string()))
          .unwrap_or(Predicate::Null)
      })
  }

  fn get_property(&self, key: &str) -> Result<Predicate, String> {
    self
      .properties
      .get(key)
      .map_or(Ok(Predicate::Null), self::json_value_to_predicate)
  }
}

impl Evaluate for JsonValue {
  fn get_geometry_type(&self) -> Result<Predicate, String> {
    self
      .as_object()
      .ok_or(format!("Evaluated json must be an object!"))?
      .get("geometry")
      .ok_or(format!("Evaluated json must contains a geometry object!"))?
      .as_object()
      .ok_or(format!("Evaluated json geometry must be an object!"))?
      .get("type")
      .ok_or(format!("Evaluated json must contains geometry.type"))
      .map(|value| {
        value
          .as_str()
          .map(|value| Predicate::String(value.to_string()))
          .unwrap_or(Predicate::Null)
      })
  }

  fn get_property(&self, key: &str) -> Result<Predicate, String> {
    self
      .as_object()
      .ok_or(format!("Evaluated json must be an object!"))?
      .get("properties")
      .ok_or(format!("Evaluated json must contains a properties object!"))?
      .as_object()
      .ok_or(format!("Evaluated json properties must be an object!"))?
      .get(key)
      .map_or(Ok(Predicate::Null), self::json_value_to_predicate)
  }
}

fn json_value_to_predicate(value: &JsonValue) -> Result<Predicate, String> {
  match value {
    JsonValue::Short(s) => Ok(Predicate::String(s.to_string())),
    JsonValue::String(s) => Ok(Predicate::String(s.to_string())),
    JsonValue::Number(_) => Ok(Predicate::Number(value.as_f64().unwrap())),
    JsonValue::Boolean(b) => Ok(Predicate::Boolean(*b)),
    _ => Ok(Predicate::Null),
  }
}

#[cfg(test)]
mod test_expression {
  use super::*;
  use json::object;
  use std::convert::TryFrom;

  #[test]
  fn evaluate_predicate() -> Result<(), String> {
    let json = object! {
      "type" => "Feature",
      "properties" => object!{
        "name:fra_x_preferred" => vec![ "Ajaccio" ],
        "wof:id" => 101748927,
        "wof:lang" => vec![ "fre" ],
        "name:eng_x_preferred" => vec![ "Ajaccio" ],
        "wof:placetype" => "localadmin",
        "bool_true" => true,
      },
      "geometry" => object!{
        "coordinates" => vec![vec![
          vec![8.585396,41.873571], vec![8.826011,41.873571], vec![8.826011,41.971536], vec![8.585396,41.968222], vec![8.585396,41.873571]
        ]],
        "type" => "Polygon"
      },
      "bbox" => vec![
        8.585396,
        41.873571,
        8.826011,
        41.971536
      ],
      "id" => 101748927,
    };
    let wof_obj = WOFGeoJSON::as_valid_wof_geojson(&json)?;

    assert_eq!(
      wof_obj.eval(&Predicate::try_from(format!("geom_type = 'Polygon'"))?)?,
      Predicate::Boolean(true)
    );
    assert_eq!(
      json.eval(&Predicate::try_from(format!("geom_type = 'Polygon'"))?)?,
      Predicate::Boolean(true)
    );

    assert_eq!(
      wof_obj.eval(&Predicate::try_from(format!(
        "wof:placetype = 'localadmin'"
      ))?)?,
      Predicate::Boolean(true)
    );
    assert_eq!(
      json.eval(&Predicate::try_from(format!(
        "wof:placetype = 'localadmin'"
      ))?)?,
      Predicate::Boolean(true)
    );

    assert_eq!(
      wof_obj.eval(&Predicate::try_from(format!("wof:id = 101748927"))?)?,
      Predicate::Boolean(true)
    );
    assert_eq!(
      json.eval(&Predicate::try_from(format!("wof:id = 101748927"))?)?,
      Predicate::Boolean(true)
    );

    assert_eq!(
      wof_obj.eval(&Predicate::try_from(format!("geom:src = 'osm'"))?)?,
      Predicate::Boolean(false)
    );
    assert_eq!(
      json.eval(&Predicate::try_from(format!("geom:src = 'osm'"))?)?,
      Predicate::Boolean(false)
    );

    assert_eq!(
      wof_obj.eval(&Predicate::try_from(format!(
        "wof:placetype = wof:placetype"
      ))?)?,
      Predicate::Boolean(true)
    );
    assert_eq!(
      json.eval(&Predicate::try_from(format!(
        "wof:placetype = wof:placetype"
      ))?)?,
      Predicate::Boolean(true)
    );

    Ok(())
  }
}

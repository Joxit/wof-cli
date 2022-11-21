mod de;
mod tokenizer;

use super::expression::de::parse;
use crate::wof::WOFGeoJSON;
use crate::JsonValue;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub enum Predicate {
  And(Box<Predicate>, Box<Predicate>),
  Or(Box<Predicate>, Box<Predicate>),
  In(Box<Predicate>, Vec<Predicate>),
  Not(Box<Predicate>),
  Eq(Box<Predicate>, Box<Predicate>),
  Neq(Box<Predicate>, Box<Predicate>),
  Variable(String),
  String(String),
  Number(f64),
  Boolean(bool),
  Null,
}

impl Predicate {
  fn eval(&self, wof: &WOFGeoJSON) -> Result<Predicate, String> {
    match self {
      Predicate::And(left, right) => Ok(Predicate::Boolean(
        left.eval(&wof)?.as_bool()? && right.eval(&wof)?.as_bool()?,
      )),
      Predicate::Or(left, right) => Ok(Predicate::Boolean(
        left.eval(&wof)?.as_bool()? || right.eval(&wof)?.as_bool()?,
      )),
      Predicate::Eq(left, right) => Ok(Predicate::Boolean(left.eval(&wof)? == right.eval(&wof)?)),
      Predicate::Not(predicate) => Ok(Predicate::Boolean(!(predicate.eval(&wof)?.as_bool()?))),
      Predicate::Boolean(b) => Ok(Predicate::Boolean(b == &true)),
      Predicate::Variable(s) => Ok(get_variable_value(&wof, s)),
      _ => Ok(self.clone()),
    }
  }

  fn as_bool(&self) -> Result<bool, String> {
    match self {
      Predicate::Boolean(b) => Ok(*b),
      _ => Err(format!("{:?} is not a boolean", self)),
    }
  }
}

impl TryFrom<String> for Predicate {
  type Error = String;
  fn try_from(predicate: String) -> Result<Self, Self::Error> {
    parse(predicate)
  }
}

fn get_variable_value(wof: &WOFGeoJSON, key: &String) -> Predicate {
  match key.as_str() {
    "geom_type" => match wof.geometry.get("type") {
      Some(value) => value
        .as_str()
        .map(|value| Predicate::String(value.to_string()))
        .unwrap_or(Predicate::Null),
      _ => Predicate::Null,
    },
    key => match wof.properties.get(key) {
      Some(value) => match value {
        JsonValue::Short(s) => Predicate::String(s.to_string()),
        JsonValue::String(s) => Predicate::String(s.to_string()),
        JsonValue::Number(_) => Predicate::Number(value.as_f64().unwrap()),
        JsonValue::Boolean(b) => Predicate::Boolean(*b),
        _ => Predicate::Null,
      },
      _ => Predicate::Null,
    },
  }
}

#[cfg(test)]
mod test_expression {
  use json::object;

  use super::*;

  #[test]
  fn create_predicate() -> Result<(), String> {
    assert_eq!(
      Predicate::try_from(format!("variable = 'true'"))?,
      Predicate::Eq(
        Box::new(Predicate::Variable("variable".to_string())),
        Box::new(Predicate::String("true".to_string()))
      )
    );

    for elem in vec![-1.90, 1.90, 0.0, 0.90, 1234.5678] {
      assert_eq!(
        Predicate::try_from(format!("variable = {}", elem))?,
        Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Number(elem))
        )
      );
    }

    for elem in vec![1, 2, -1, -100] {
      assert_eq!(
        Predicate::try_from(format!("variable = {}", elem))?,
        Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Number(elem.into()))
        )
      );
    }

    for elem in vec![true, false] {
      assert_eq!(
        Predicate::try_from(format!("variable = {}", elem))?,
        Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Boolean(elem))
        )
      );
    }

    for elem in vec!["null", "Null", "NULL"] {
      assert_eq!(
        Predicate::try_from(format!("variable = {}", elem))?,
        Predicate::Eq(
          Box::new(Predicate::Variable("variable".to_string())),
          Box::new(Predicate::Null)
        )
      );
    }

    for elem in vec!["wof:placetype", "geom_type"] {
      assert_eq!(
        Predicate::try_from(format!("{} = true", elem))?,
        Predicate::Eq(
          Box::new(Predicate::Variable(elem.to_string())),
          Box::new(Predicate::Boolean(true))
        )
      );
    }
    Ok(())
  }

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
      Predicate::try_from(format!("geom_type = 'Polygon'"))?.eval(&wof_obj)?,
      Predicate::Boolean(true)
    );

    assert_eq!(
      Predicate::try_from(format!("wof:placetype = 'localadmin'"))?.eval(&wof_obj)?,
      Predicate::Boolean(true)
    );

    assert_eq!(
      Predicate::try_from(format!("wof:id = 101748927"))?.eval(&wof_obj)?,
      Predicate::Boolean(true)
    );

    assert_eq!(
      Predicate::try_from(format!("geom:src = 'osm'"))?.eval(&wof_obj)?,
      Predicate::Boolean(false)
    );

    assert_eq!(
      Predicate::try_from(format!("wof:placetype = wof:placetype"))?.eval(&wof_obj)?,
      Predicate::Boolean(true)
    );

    Ok(())
  }
}

pub use json::object::Object;
pub use json::JsonValue;

pub trait JsonUtils {
  fn as_json_value(&self) -> &JsonValue;
  fn as_mut_json_value(&mut self) -> &mut JsonValue;
  fn assert_is_object(&self) -> Result<(), String> {
    match &self.as_json_value() {
      JsonValue::Object(_) => Ok(()),
      _ => Err(format!(
        "This is not an object but a {}",
        self.type_as_string()
      )),
    }
  }
  fn assert_is_array(&self) -> Result<(), String> {
    match &self.as_json_value() {
      JsonValue::Array(_) => Ok(()),
      _ => Err(format!(
        "This is not an array but a {}",
        self.type_as_string()
      )),
    }
  }
  fn assert_is_number(&self) -> Result<(), String> {
    match &self.as_json_value() {
      JsonValue::Number(_) => Ok(()),
      _ => Err(format!(
        "This is not a number but a {}",
        self.type_as_string()
      )),
    }
  }
  fn assert_is_string(&self) -> Result<(), String> {
    match &self.as_json_value() {
      JsonValue::String(_) => Ok(()),
      JsonValue::Short(_) => Ok(()),
      _ => Err(format!(
        "This is not a String but a {}",
        self.type_as_string()
      )),
    }
  }
  fn as_object(&self) -> Option<&Object> {
    match &self.as_json_value() {
      JsonValue::Object(ref obj) => Some(obj),
      _ => None,
    }
  }
  fn as_mut_object(&mut self) -> Option<&mut Object> {
    match self.as_mut_json_value() {
      JsonValue::Object(ref mut obj) => Some(obj),
      _ => None,
    }
  }
  fn keys(&self) -> Vec<String> {
    match &self.as_json_value() {
      JsonValue::Object(ref obj) => {
        let mut keys = Vec::new();
        for (k, _) in obj.iter() {
          keys.push(k.to_string());
        }
        keys
      }
      _ => Vec::new(),
    }
  }
  fn as_array(&self) -> Option<&Vec<JsonValue>> {
    match &self.as_json_value() {
      JsonValue::Array(ref array) => Some(array),
      _ => None,
    }
  }
  fn type_as_string(&self) -> &str {
    match &self.as_json_value() {
      JsonValue::Object(_) => "Object",
      JsonValue::Array(_) => "Array",
      JsonValue::Number(_) => "Number",
      JsonValue::String(_) => "String",
      JsonValue::Short(_) => "Short",
      JsonValue::Boolean(_) => "Boolean",
      JsonValue::Null => "Null",
    }
  }
}

impl JsonUtils for JsonValue {
  fn as_json_value(&self) -> &JsonValue {
    &self
  }
  fn as_mut_json_value(&mut self) -> &mut JsonValue {
    self
  }
}

pub trait GeoJsonUtils {
  fn as_json_value(&self) -> &JsonValue;
  fn as_mut_json_value(&mut self) -> &mut JsonValue;

  fn as_geom_point(&self) -> Option<Vec<f64>> {
    if !self.as_json_value().is_array() {
      return None;
    }
    let array = self.as_json_value().as_array().unwrap();
    let mut point = vec![];
    for p in array.iter() {
      if p.is_number() {
        point.push(p.as_f64().unwrap())
      } else {
        return None;
      }
    }
    Some(point)
  }

  fn as_geom_multi_point(&self) -> Option<Vec<Vec<f64>>> {
    if !self.as_json_value().is_array() {
      return None;
    }
    let array = self.as_json_value().as_array().unwrap();
    let mut multi_point = vec![];
    for point in array.iter() {
      if let Some(point) = point.as_geom_point() {
        multi_point.push(point)
      } else {
        return None;
      }
    }
    Some(multi_point)
  }

  fn as_geom_line(&self) -> Option<Vec<Vec<f64>>> {
    self.as_geom_multi_point()
  }

  fn as_geom_multi_line(&self) -> Option<Vec<Vec<Vec<f64>>>> {
    if !self.as_json_value().is_array() {
      return None;
    }
    let array = self.as_json_value().as_array().unwrap();
    let mut multi_line = vec![];
    for line in array.iter() {
      if let Some(line) = line.as_geom_line() {
        multi_line.push(line)
      } else {
        return None;
      }
    }
    Some(multi_line)
  }

  fn as_geom_polygon(&self) -> Option<Vec<Vec<Vec<f64>>>> {
    self.as_geom_multi_line()
  }

  fn as_geom_multi_polygon(&self) -> Option<Vec<Vec<Vec<Vec<f64>>>>> {
    if !self.as_json_value().is_array() {
      return None;
    }
    let array = self.as_json_value().as_array().unwrap();
    let mut multi_polygon = vec![];
    for polygon in array.iter() {
      if let Some(polygon) = polygon.as_geom_polygon() {
        multi_polygon.push(polygon)
      } else {
        return None;
      }
    }
    Some(multi_polygon)
  }
}

impl GeoJsonUtils for JsonValue {
  fn as_json_value(&self) -> &JsonValue {
    &self
  }
  fn as_mut_json_value(&mut self) -> &mut JsonValue {
    self
  }
}

#[cfg(test)]
mod test_geojson {
  use super::*;
  use json::array;

  #[test]
  pub fn point() {
    let point = array![30.0, 10.0];
    assert_eq!(point.as_geom_point(), Some(vec![30.0, 10.0]));
    assert_eq!(point.as_geom_line(), None);
    assert_eq!(point.as_geom_polygon(), None);
  }

  #[test]
  pub fn line() {
    let line = array![array![30.0, 10.0], array![10.0, 30.0], array![40.0, 40.0]];
    let expect = Some(vec![vec![30.0, 10.0], vec![10.0, 30.0], vec![40.0, 40.0]]);
    assert_eq!(line.as_geom_line(), expect);
    assert_eq!(line.as_geom_multi_point(), expect);
    assert_eq!(line.as_geom_point(), None);
    assert_eq!(line.as_geom_polygon(), None);
  }

  #[test]
  pub fn polygon() {
    let polygon = array![array![
      array![30.0, 10.0],
      array![40.0, 40.0],
      array![20.0, 40.0],
      array![10.0, 20.0],
      array![30.0, 10.0]
    ]];
    let expect = Some(vec![vec![
      vec![30.0, 10.0],
      vec![40.0, 40.0],
      vec![20.0, 40.0],
      vec![10.0, 20.0],
      vec![30.0, 10.0],
    ]]);
    assert_eq!(polygon.as_geom_polygon(), expect);
    assert_eq!(polygon.as_geom_multi_line(), expect);
    assert_eq!(polygon.as_geom_point(), None);
    assert_eq!(polygon.as_geom_line(), None);
  }

  #[test]
  pub fn polygon_inner() {
    let polygon = array![
      array![
        array![35.0, 10.0],
        array![45.0, 45.0],
        array![15.0, 40.0],
        array![10.0, 20.0],
        array![35.0, 10.0]
      ],
      array![
        array![20.0, 30.0],
        array![35.0, 35.0],
        array![30.0, 20.0],
        array![20.0, 30.0]
      ]
    ];
    let expect = Some(vec![
      vec![
        vec![35.0, 10.0],
        vec![45.0, 45.0],
        vec![15.0, 40.0],
        vec![10.0, 20.0],
        vec![35.0, 10.0],
      ],
      vec![
        vec![20.0, 30.0],
        vec![35.0, 35.0],
        vec![30.0, 20.0],
        vec![20.0, 30.0],
      ],
    ]);
    assert_eq!(polygon.as_geom_polygon(), expect);
    assert_eq!(polygon.as_geom_multi_line(), expect);
    assert_eq!(polygon.as_geom_point(), None);
    assert_eq!(polygon.as_geom_line(), None);
  }

  #[test]
  pub fn multi_polygon() {
    let multi_polygon = array![
      array![array![
        array![40.0, 40.0],
        array![20.0, 45.0],
        array![45.0, 30.0],
        array![40.0, 40.0]
      ]],
      array![
        array![
          array![20.0, 35.0],
          array![10.0, 30.0],
          array![10.0, 10.0],
          array![30.0, 5.0],
          array![45.0, 20.0],
          array![20.0, 35.0]
        ],
        array![
          array![30.0, 20.0],
          array![20.0, 15.0],
          array![20.0, 25.0],
          array![30.0, 20.0]
        ]
      ]
    ];
    let expect = Some(vec![
      vec![vec![
        vec![40.0, 40.0],
        vec![20.0, 45.0],
        vec![45.0, 30.0],
        vec![40.0, 40.0],
      ]],
      vec![
        vec![
          vec![20.0, 35.0],
          vec![10.0, 30.0],
          vec![10.0, 10.0],
          vec![30.0, 5.0],
          vec![45.0, 20.0],
          vec![20.0, 35.0],
        ],
        vec![
          vec![30.0, 20.0],
          vec![20.0, 15.0],
          vec![20.0, 25.0],
          vec![30.0, 20.0],
        ],
      ],
    ]);
    assert_eq!(multi_polygon.as_geom_multi_polygon(), expect);
    assert_eq!(multi_polygon.as_geom_point(), None);
    assert_eq!(multi_polygon.as_geom_line(), None);
    assert_eq!(multi_polygon.as_geom_polygon(), None);
  }
}

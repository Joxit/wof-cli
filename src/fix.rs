use crate::utils::JsonUtils;
use crate::JsonValue;

pub struct Fix {
  population: bool,
}

const POPULATION_PROPERTIES: &[&str] = &[
  "mz:population",
  "wof:population",
  "wk:population",
  "gn:population",
  "gn:pop",
  "qs:pop",
  "qs:gn_pop",
  "zs:pop10",
  "meso:pop",
  "statoids:population",
  "ne:pop_est",
];

impl Fix {
  pub fn new() -> Self {
    Fix { population: true }
  }

  pub fn fix(&self, obj: &mut JsonValue) -> Result<bool, String> {
    let mut has_changed = false;
    if self.population {
      has_changed = has_changed || self.fix_population_mut(obj)?;
    }

    Ok(has_changed)
  }

  fn fix_population_mut(&self, obj: &mut JsonValue) -> Result<bool, String> {
    let mut has_changed = false;
    let properties = obj
      .as_mut_object()
      .ok_or(format!("Input sinot a GeoJSON"))?
      .get_mut("properties")
      .ok_or(format!("`properties` key not found in GeoJSON"))?
      .as_mut_object()
      .ok_or(format!("`properties` key is not an object in GeoJSON"))?;

    POPULATION_PROPERTIES
      .iter()
      .for_each(|key| match properties.get(key) {
        Some(JsonValue::String(s)) => {
          if let Some(new_value) = fix_strigified_number(s) {
            properties.insert(key, JsonValue::from(new_value));
            has_changed = true;
          }
        }
        Some(JsonValue::Short(s)) => {
          if let Some(new_value) = fix_strigified_number(&s.to_string()) {
            properties.insert(key, JsonValue::from(new_value));
            has_changed = true;
          }
        }
        _ => {}
      });
    Ok(has_changed)
  }
}

fn fix_strigified_number(value: &String) -> Option<String> {
  let new_value = value.replace(",", "");
  if *value != new_value {
    Some(new_value)
  } else {
    None
  }
}

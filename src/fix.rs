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

  pub fn fix(&self, obj: &mut JsonValue) -> Result<(), String> {
    if self.population {
      self.fix_population_mut(obj)?;
    }

    Ok(())
  }

  fn fix_population_mut(&self, obj: &mut JsonValue) -> Result<(), String> {
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
        Some(JsonValue::String(s)) => properties.insert(key, Fix::string_to_number(s)),
        Some(JsonValue::Short(s)) => properties.insert(key, Fix::string_to_number(&s.to_string())),
        _ => {}
      });
    Ok(())
  }

  fn string_to_number(s: &String) -> JsonValue {
    JsonValue::from(s.replace(",", ""))
  }
}

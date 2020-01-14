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

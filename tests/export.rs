use json::{array, object, parse};
use std::fs;
use wof::export::export_json_value;

#[cfg(test)]
mod export_json_value {
  use super::*;
  #[test]
  fn export_array_should_fail() {
    let json = array![];
    assert_eq!(export_json_value(json).is_err(), true);
  }

  #[test]
  fn export_empty_should_fail() {
    let json = object! {};
    assert_eq!(export_json_value(json).is_err(), true);
  }

  #[test]
  fn export_empty_properties_should_fail() {
    let json = object! { "properties" => object! {} };
    assert_eq!(export_json_value(json).is_err(), true);
  }
}

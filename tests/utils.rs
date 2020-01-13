#[macro_use]
extern crate json;
use std::path::Path;
use wof::utils::{self, JsonObject, JsonUtils};
#[test]
fn json_utils_accessibility() {
  assert_eq!(object! {}.assert_is_object(), Ok(()));
  assert_eq!(object! {}.as_object(), Some(&JsonObject::new()));
}

#[test]
fn path_utils_accessibility() {
  assert_eq!(
    utils::id_to_data_path_folder(890442055),
    Path::new("data/890/442/055").to_path_buf()
  );
  assert_eq!(
    utils::id_to_path_folder(890442055),
    Path::new("890/442/055").to_path_buf()
  );
  assert_eq!(
    utils::id_to_data_path_geojson(890442055),
    Path::new("data/890/442/055/890442055.geojson").to_path_buf()
  );
  assert_eq!(
    utils::id_to_path_geojson(890442055),
    Path::new("890/442/055/890442055.geojson").to_path_buf()
  );
}

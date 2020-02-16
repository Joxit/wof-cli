#[macro_use]
extern crate json;

use json::Null;
use wof::json_to_writer_pretty;

#[test]
pub fn serialize_first_level_wof_geojson_with_null() {
  let t = object! {
    "type"=> Null,
    "properties"=> Null,
    "geometry"=> Null,
    "bbox"=> Null,
    "id"=> Null,
  };
  let mut vec: Vec<u8> = Vec::new();
  assert!(json_to_writer_pretty(&t, &mut vec).is_ok());
  assert_eq!(
    String::from_utf8(vec).unwrap(),
    r#"{
  "id": null,
  "type": null,
  "properties": null,
  "bbox": null,
  "geometry": null
}"#
  );
}

#[test]
pub fn serialize_first_level_wof_geojson_with_content() {
  let t = object! {
    "type"=>"Feature",
    "properties"=> object!{
      "name:fra_x_preferred" => vec![
        "Ajaccio"
      ],
      "wof:id"=>101748927,
      "wof:lang" => vec![
        "fre"
      ],
      "name:eng_x_preferred" => vec![
        "Ajaccio"
      ],
    },
    "geometry"=> object!{
      "coordinates" => vec![vec![
        vec![8.585396,41.873571],
        vec![8.826011,41.873571],
        vec![8.826011,41.971536],
        vec![8.585396,41.968222],
        vec![8.585396,41.873571]
      ]],
      "type" => "Polygon"
    },
    "bbox"=> vec![
      8.585396,
      41.873571,
      8.826011,
      41.971536
    ],
    "id"=> 101748927,
  };
  let mut vec: Vec<u8> = Vec::new();
  assert!(json_to_writer_pretty(&t, &mut vec).is_ok());
  assert_eq!(
    String::from_utf8(vec).unwrap(),
    r#"{
  "id": 101748927,
  "type": "Feature",
  "properties": {
    "name:eng_x_preferred":[
      "Ajaccio"
    ],
    "name:fra_x_preferred":[
      "Ajaccio"
    ],
    "wof:id":101748927,
    "wof:lang":[
      "fre"
    ]
  },
  "bbox": [
    8.585396,
    41.873571,
    8.826011,
    41.971536
],
  "geometry": {"coordinates":[[[8.585396,41.873571],[8.826011,41.873571],[8.826011,41.971536],[8.585396,41.968222],[8.585396,41.873571]]],"type":"Polygon"}
}"#
  );
}
